use crate::vehicle::Vehicle;

pub struct Car {
    maker: String,
    model: String,
    year: i32,
    speed: u32,
}
impl Car {
    pub fn new(maker: impl Into<String>, model: impl Into<String>, year: i32, speed: u32) -> Self {
        Self {
            maker: maker.into(),
            model: model.into(),
            year,
            speed,
        }
    }

    pub fn set_speed(&mut self, speed: u32) {
        self.speed = speed;
    }

    pub fn maker(&self) -> &str {
        &self.maker
    }

    pub fn model(&self) -> &str {
        &self.model
    }

    pub fn year(&self) -> i32 {
        self.year
    }

    pub fn speed(&self) -> u32 {
        self.speed
    }
}

impl Vehicle for Car {
    fn drive(&self) {
        println!(
            "'CAR' {} {} ({}) is driving at {} km/h.",
            self.maker, self.model, self.year, self.speed
        );
    }

    fn name(&self) -> &str {
        &self.maker
    }
}
