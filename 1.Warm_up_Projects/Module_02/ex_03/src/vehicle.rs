pub trait Vehicle {
    fn drive(&self);
    fn name(&self) -> &str;
}
pub fn static_dispatch<T: Vehicle>(t: T) {
    println!("From static dispatch:");
    t.drive();
}
