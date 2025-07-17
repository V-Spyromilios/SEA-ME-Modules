mod car;
mod racetrack;

use std::sync::{Arc, Mutex};

use car::{Car, spawn_car_thread};
use racetrack::RaceTrack;
use slint::ComponentHandle;
slint::include_modules!();

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ui = MainWindow::new()?;

    let racetrack = Arc::new(RaceTrack::new(100));
    let cars = vec![
        Arc::new(Mutex::new(Car::new("Car A".to_string(), 3))),
        Arc::new(Mutex::new(Car::new("Car B".to_string(), 2))),
        Arc::new(Mutex::new(Car::new("Car C".to_string(), 1))),
    ];

    let ui_handle_for_race = ui.as_weak();
    let cars_for_race = cars.clone();
    let racetrack_for_race = racetrack.clone();

    let ui_handle = ui.as_weak();

    ui.on_start_race(move || {
        // Reset positions
        for car in &cars_for_race {
            let mut c = car.lock().unwrap();
            c.position = 0;
        }

        if let Some(ui) = ui_handle_for_race.upgrade() {
            ui.set_car1_position(0);
            ui.set_car2_position(0);
            ui.set_car3_position(0);
        }

        for (i, car) in cars_for_race.iter().enumerate() {
            let car_clone = Arc::clone(car);
            let track_clone = Arc::clone(&racetrack_for_race);
            let ui_handle = ui_handle_for_race.clone();

            spawn_car_thread(car_clone, track_clone, move |pos| {
                let ui_handle_inner = ui_handle.clone();
                slint::invoke_from_event_loop(move || {
                    if let Some(ui) = ui_handle_inner.upgrade() {
                        match i {
                            0 => ui.set_car1_position(pos as i32),
                            1 => ui.set_car2_position(pos as i32),
                            2 => ui.set_car3_position(pos as i32),
                            _ => {}
                        }
                    }
                })
                .unwrap();
            });
        }
    });

    ui.run()?;
    Ok(())
}
