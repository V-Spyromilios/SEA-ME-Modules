pub trait Vehicle {
    fn drive(&self);
    fn name(&self) -> &str;
}
