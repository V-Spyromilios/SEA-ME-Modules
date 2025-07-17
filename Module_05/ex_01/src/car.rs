use crate::racetrack::RaceTrack;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Car {
    pub name: String,
    pub position: usize,
    pub speed: usize,
    pub direction: isize, // +1 for forward movement
}

impl Car {
    pub fn new(name: String, speed: usize) -> Self {
        Car {
            name: name.to_string(),
            position: 0,
            speed,
            direction: 1,
        }
    }

    pub fn move_forward(&mut self) {
        let delta = self.speed as isize * self.direction;
        self.position = (self.position as isize + delta).max(0) as usize;
    }
}
pub fn spawn_car_thread(
    car: Arc<Mutex<Car>>,
    racetrack: Arc<RaceTrack>,
    update_ui: impl Fn(usize) + Send + 'static,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        loop {
            {
                let mut car = car.lock().unwrap();
                if racetrack.is_finished(car.position) {
                    println!("{} has finished the race!", car.name);
                    break;
                }
                car.move_forward();
                update_ui(car.position);
                println!("{} moved to position {}", car.name, car.position);
            }
            thread::sleep(Duration::from_millis(100));
        }
    })
}
