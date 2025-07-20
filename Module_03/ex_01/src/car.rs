use crate::part::Part;
use crate::parts::brake::Brake;
use crate::parts::engine::Engine;
use crate::parts::transmission::Transmission;
use crate::parts::wheel::Wheel;
use crate::vehicle::Vehicle;

#[derive(Debug, Clone)]
pub enum CarModel {
    Custom(String),
    ID(String), // For VW ID electric vehicles
}

pub struct Car {
    model: CarModel,
    engine: Engine,
    wheels: Vec<Wheel>,
    brakes: Vec<Brake>,
    transmission: Transmission,
}

impl Car {
    pub fn new(
        model: CarModel,
        engine: Engine,
        wheels: Vec<Wheel>,
        brakes: Vec<Brake>,
        transmission: Transmission,
    ) -> Self {
        Self {
            model,
            engine,
            wheels,
            brakes,
            transmission,
        }
    }
}

impl Vehicle for Car {
    fn start(&self) {
        println!("Starting {:?}...", self.model);
    }

    fn print_parts(&self) {
        self.engine.print();
        for wheel in &self.wheels {
            wheel.print();
        }
        for brake in &self.brakes {
            brake.print();
        }
        self.transmission.print();
    }
}
