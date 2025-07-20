use crate::part::Part;

#[derive(Debug, Clone)]
pub struct Engine {
    horsepower: u32,
    displacement: f32, // in liters
}

impl Engine {
    pub fn new(horsepower: u32, displacement: f32) -> Self {
        Self {
            horsepower,
            displacement,
        }
    }
}

impl Part for Engine {
    fn print(&self) {
        println!(
            "From Engine: {} HP, {:.1}L",
            self.horsepower, self.displacement
        );
    }
}
