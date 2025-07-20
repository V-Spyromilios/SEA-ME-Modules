#[derive(Debug, Clone)]
pub struct Car {
    name: String,
    speed_kmh: u32,
}

impl Car {
    pub fn new(name: impl Into<String>, speed_kmh: u32) -> Self {
        Self {
            name: name.into(),
            speed_kmh,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn speed(&self) -> u32 {
        //Copy Primitive type.
        self.speed_kmh
    }

    pub fn drive(&self) {
        println!("{} is flying at {} km/h!", self.name, self.speed_kmh);
    }
}
