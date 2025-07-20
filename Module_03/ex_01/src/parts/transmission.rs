use crate::part::Part;

#[derive(Debug, Clone)]
pub struct Transmission {
    gears: u8,
    automatic: bool,
}

impl Transmission {
    pub fn new(gears: u8, automatic: bool) -> Self {
        Self { gears, automatic }
    }
}

impl Part for Transmission {
    fn print(&self) {
        let kind = if self.automatic {
            "Automatic"
        } else {
            "Manual"
        };
        println!("Transmission: {}-speed {}", self.gears, kind);
    }
}
