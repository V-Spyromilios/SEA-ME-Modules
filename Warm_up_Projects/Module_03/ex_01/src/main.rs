mod car;
mod part;
mod parts;
mod vehicle;

use crate::car::{Car, CarModel};
use parts::brake::{Brake, BrakeKind, BrakeMaterial};
use parts::engine::Engine;
use parts::transmission::Transmission;
use parts::wheel::{Wheel, WheelType};
use vehicle::Vehicle;

fn main() {
    let garage: Vec<Box<dyn Vehicle>> = vec![
        Box::new(Car::new(
            CarModel::Custom("Ferrari F8 Tributo".into()),
            Engine::new(710, 3.9),
            vec![Wheel::new(20, WheelType::Alloy); 4],
            vec![Brake::new(BrakeKind::Disc, BrakeMaterial::Carbon); 4],
            Transmission::new(7, true),
        )),
        Box::new(Car::new(
            CarModel::ID("Volkswagen ID.3".into()),
            Engine::new(204, 0.0), // Electric, no displacement
            vec![Wheel::new(18, WheelType::Steel); 4],
            vec![Brake::new(BrakeKind::Drum, BrakeMaterial::Steel); 4],
            Transmission::new(1, true),
        )),
        Box::new(Car::new(
            CarModel::ID("Volkswagen ID.4".into()),
            Engine::new(286, 0.0),
            vec![Wheel::new(19, WheelType::Steel); 4],
            vec![Brake::new(BrakeKind::Regenerative, BrakeMaterial::Steel); 4],
            Transmission::new(1, true),
        )),
        Box::new(Car::new(
            CarModel::ID("Volkswagen ID.7 Tourer".into()),
            Engine::new(340, 0.0),
            vec![Wheel::new(20, WheelType::Alloy); 4],
            vec![Brake::new(BrakeKind::Regenerative, BrakeMaterial::Carbon); 4],
            Transmission::new(1, true),
        )),
    ];

    for (i, car) in garage.iter().enumerate() {
        println!("--- Garage Slot {} ---", i + 1);
        car.start();
        car.print_parts();
    }
}
