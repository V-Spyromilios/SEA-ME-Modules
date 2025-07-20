use crate::part::Part;
#[derive(Debug, Clone)]
pub enum BrakeKind {
    Disc,
    Drum,
    Regenerative,
}

#[derive(Debug, Clone)]
pub enum BrakeMaterial {
    Ceramic,
    Steel,
    Carbon,
}

#[derive(Debug, Clone)]
pub struct Brake {
    kind: BrakeKind,
    material: BrakeMaterial,
}

impl Brake {
    pub fn new(kind: BrakeKind, material: BrakeMaterial) -> Self {
        Self { kind, material }
    }
}

impl Part for Brake {
    fn print(&self) {
        println!("From Brake: {:?} ({:?})", self.kind, self.material);
    }
}
