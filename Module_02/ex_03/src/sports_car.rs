use crate::car::Car;
use crate::vehicle::Vehicle;

pub struct SportsCar {
    base: Car,
    top_speed: u32,
}

impl SportsCar {
    pub fn new(
        maker: impl Into<String>,
        model: impl Into<String>,
        year: i32,
        top_speed: u32,
    ) -> Self {
        let base = Car::new(maker, model, year, 0); // start speed at 0
        Self { base, top_speed }
    }

    pub fn boost(&mut self) {
        self.base.set_speed(self.top_speed);
    }

    pub fn top_speed(&self) -> u32 {
        self.top_speed
    }

    pub fn base(&self) -> &Car {
        &self.base
    }

    pub fn base_mut(&mut self) -> &mut Car {
        &mut self.base
    }
}

impl Vehicle for SportsCar {
    fn drive(&self) {
        self.base.drive();
        println!("Boosted top speed: {} km/h!", self.top_speed);
    }

    fn name(&self) -> &str {
        self.base.name()
    }
}
