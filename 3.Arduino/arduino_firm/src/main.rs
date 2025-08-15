#![feature(abi_avr_interrupt)]
#![no_std]
#![no_main]

use panic_halt as _;
use ufmt::uwriteln;
use avr_device::interrupt;
use embedded_can::Frame as _;

use embedded_hal_bus::spi::ExclusiveDevice;
use embedded_can::{Id, StandardId};
use mcp2515::{frame::CanFrame, CanSpeed, McpSpeed, regs::OpMode, MCP2515};

const SLOTS_PER_REV: u32 = 20;
const DIAMETER_MM: u32 = 80;
const CIRCUM_MM: u32 = (DIAMETER_MM * 31416) / 10000; // π≈3.1416

static mut PULSES: u16 = 0;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, 115200);

    // Encoder sensor on D2 (INT0)
    let _sensor = pins.d2.into_pull_up_input();

    // ----------- MCP2515 over SPI -----------
    // SPI pins: SCK=D13, MOSI=D11, MISO=D12, CS=D10

    let (spi, cs) = arduino_hal::Spi::new(
        dp.SPI,
        pins.d13.into_output(),            // SCK
        pins.d11.into_output(),            // MOSI
        pins.d12.into_pull_up_input(),     // MISO
        pins.d10.into_output(),            // CS
        arduino_hal::spi::Settings {
            data_order: arduino_hal::spi::DataOrder::MostSignificantFirst,
            clock: arduino_hal::spi::SerialClockRate::OscfOver2, // fast enough for MCP2515
            mode: embedded_hal::spi::MODE_0,
        },
    );

    // Wrap bus+CS into a SpiDevice (what the driver expects)
    let mut delay = arduino_hal::Delay::new();
    let spi_dev = ExclusiveDevice::new(spi, cs, arduino_hal::Delay::new()).unwrap();
    let mut can = MCP2515::new(spi_dev);

    // Choose the correct MCP2515 oscillator here (8 or 16 MHz)
    let mcp_clk = McpSpeed::MHz8; // change to McpSpeed::MHz16 if your crystal says 16.000

    can.init(
        &mut delay,
        mcp2515::Settings {
            mode: OpMode::Normal,
            can_speed: CanSpeed::Kbps500, // or Kbps250/Kbps125 as needed
            mcp_speed: mcp_clk,
            clkout_en: false,
        },
    ).unwrap();

    // ----------- Enable INT0 for encoder -----------
    unsafe {
        dp.EXINT.eicra.modify(|_, w| w.isc0().bits(0b10));   // falling edge
        dp.EXINT.eifr.write(|w| w.bits(1 << 0));             // clear pending
        dp.EXINT.eimsk.modify(|_, w| w.int0().set_bit());    // enable INT0
        interrupt::enable();
    }

    uwriteln!(&mut serial, "CAN up @500kbps, encoder running\r").ok();

    loop {
        arduino_hal::delay_ms(1000);

        // Take pulse count safely
        let pulses = interrupt::free(|_| unsafe {
            let p = PULSES as u32;
            PULSES = 0;
            p
        });

        let rpm = (pulses * 60) / SLOTS_PER_REV;
        let mm_per_s = (pulses * CIRCUM_MM) / SLOTS_PER_REV;
        let kmh_x100 = (mm_per_s * 9) / 25; // 1 mm/s = 0.0036 km/h

        // --- Pack frame: ID 0x101, 4-byte payload: speed_kmh_x100, rpm (big-endian) ---
        let id = Id::Standard(StandardId::new(0x101).unwrap());

        let speed_be = (kmh_x100 as u16).to_be_bytes();
        let rpm_be = (rpm as u16).to_be_bytes();

        let data = [speed_be[0], speed_be[1], rpm_be[0], rpm_be[1]]; // DLC=4
        let frame = CanFrame::new(id, &data).unwrap();

        // Transmit
        if let Err(_e) = can.send_message(frame) {
            // optional: print an error or blink LED
            uwriteln!(&mut serial, "CAN TX error\r").ok();
        } else {
            uwriteln!(
                &mut serial,
                "TX id=0x101 speed={}.{} km/h rpm={}\r",
                kmh_x100 / 100,
                kmh_x100 % 100,
                rpm
            ).ok();
        }
    }
}

#[avr_device::interrupt(atmega328p)]
fn INT0() {
    unsafe { PULSES = PULSES.wrapping_add(1); }
}
