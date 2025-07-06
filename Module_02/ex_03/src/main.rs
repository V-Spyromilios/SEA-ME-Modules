/*
* OOP Concept       Rust Equivalent

Abstraction         trait Vehicle { fn drive(&self); }
Encapsulation       private fields + pub getters/setters
Inheritance         Composition + trait polymorphism
Polymorphism        Box<dyn Vehicle> or enums
Modularity          split into multiple files / mod structure

*/

mod car;
mod sports_car;
mod vehicle;

use crate::car::Car;
use crate::sports_car::SportsCar;
use crate::vehicle::Vehicle;

fn main() {
    let mut corolla = Car::new("Toyota", "Corolla", 2020, 180);
    corolla.drive();
    corolla.set_speed(200);
    corolla.drive();

    let mut ferrari = SportsCar::new("Ferrari", "F8 Tributo", 2022, 340);
    ferrari.drive();
    ferrari.boost();
    ferrari.drive();

    // --- Polymorphism with trait objects ---
    let garage: Vec<Box<dyn Vehicle>> = vec![
        Box::new(Car::new("Honda", "Civic", 2018, 140)),
        Box::new(SportsCar::new("Lamborghini", "Huracán", 2023, 325)),
    ];

    for vehicle in &garage {
        println!("Now driving: {}", vehicle.name());
        vehicle.drive();
    }
}
