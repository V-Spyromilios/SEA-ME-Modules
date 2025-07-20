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
use crate::vehicle::{Vehicle, static_dispatch};

fn main() {
    let mut corolla = Car::new("Toyota", "Corolla", 2020, 180);
    corolla.drive();
    corolla.set_speed(200);
    corolla.drive();

    let mut ferrari = SportsCar::new("Ferrari", "F8 Tributo", 2022, 340);
    ferrari.drive();
    ferrari.boost();
    ferrari.drive();

    // 1. dynamic polymorphism with trait objects
    let garage: Vec<Box<dyn Vehicle>> = vec![
        Box::new(Car::new("Honda", "Civic", 2018, 140)),
        Box::new(SportsCar::new("Lamborghini", "Huracán", 2023, 325)),
    ];

    for vehicle in &garage {
        println!("Now driving: {}", vehicle.name());
        vehicle.drive();
    }
    // 2. static dispatch with trait Bounds
    static_dispatch(ferrari);
}

/* 1
    Dynamic dispatch:
- Uses trait objects: Box<dyn Vehicle>, &dyn Vehicle. Fat pointer -to the data and to the vtable
- Allows storing different types that implement Vehicle in the same collection (Vec<Box<dyn Vehicle>>)
- The method calls are resolved at runtime using vtables (like virtual methods in C++). -> smaller binary, impact on performance.
*/

/* 2
    Static dispatch:
  -  Uses generics: fn static_dispatch<T: Vehicle>(t: T)
  - The compiler generates a separate version of the function for each concrete type T. -> larger binary, better performance
  - Zero-cost abstraction, faster at runtime
  - The type must be known exactly at compile time
*/
