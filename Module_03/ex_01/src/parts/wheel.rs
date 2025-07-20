use crate::part::Part;

#[derive(Debug, Clone)]
pub enum WheelType {
    Alloy,
    Steel,
    CarbonFiber,
}

#[derive(Debug, Clone)]
pub struct Wheel {
    diameter: u8, // inches
    type_: WheelType,
}

impl Wheel {
    pub fn new(diameter: u8, type_: WheelType) -> Self {
        Self { diameter, type_ }
    }
}

impl Part for Wheel {
    fn print(&self) {
        println!("Wheel: {}\" {:?}", self.diameter, self.type_);
    }
}
