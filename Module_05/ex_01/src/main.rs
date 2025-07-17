mod car;
mod racetrack;

use std::sync::{Arc, Mutex};

use car::{Car, spawn_car_thread};
use racetrack::RaceTrack;
use slint::SharedString;

slint::include_modules!();

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ui = AppWindow::new()?;

    let racetrack = Arc::new(RaceTrack::new(100));
    let cars = vec![
        Arc::new(Mutex::new(Car::new("Car A".to_string(), 3))),
        Arc::new(Mutex::new(Car::new("Car B".to_string(), 2))),
        Arc::new(Mutex::new(Car::new("Car C".to_string(), 1))),
    ];

    let car_1_ui = ui.global::<CarPositions>().get_car1_position();
    let car_2_ui = ui.global::<CarPositions>().get_car2_position();
    let car_3_ui = ui.global::<CarPositions>().get_car3_position();

    let ui_handle = ui.as_weak();
    ui.on_start_race(move || {
        let ui = ui_handle.unwrap();

        let car_positions = vec![car_1_ui, car_2_ui, car_3_ui];
        for (i, car) in cars.iter().enumerate() {
            let car_clone = Arc::clone(car);
            let track_clone = Arc::clone(&racetrack);
            let pos_handle = car_positions[i].clone();

            spawn_car_thread(car_clone, track_clone, move |pos| {
                pos_handle.set(pos as i32);
            });
        }
    });

    ui.run()?;
    Ok(())
}
