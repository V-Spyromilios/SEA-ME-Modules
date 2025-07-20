# Rust Warm-up Exercise 02: Software-Defined Car (OOP in Rust)

This project is a complete Rust implementation of a **software-defined vehicle architecture**, showcasing all key **Object-Oriented Programming (OOP)** principles — expressed idiomatically through Rust's type system, traits, and composition.

---

## Goal

Simulate the design of a `Car` and a `SportsCar` using OOP features:

- **Abstraction**
- **Encapsulation**
- **Inheritance**
- **Polymorphism**
- **Modularity**

Rust doesn’t use classical class-based inheritance. Instead, it encourages **composition and traits** — powerful alternatives to achieve the same design goals.

---

## Architecture

### `Vehicle` Trait (Abstraction)

```rust
pub trait Vehicle {
    fn drive(&self);
    fn name(&self) -> &str;
}
```

All vehicles expose a uniform interface with drive() and name().
This allows static or dynamic dispatch, and encapsulates behavior behind a common abstraction.

### `Car` and `SportsCar` Implementations

```rust
pub struct Car {
    maker: String,
    model: String,
    year: i32,
    speed: u32,
}
```

> - Implements Vehicle
> - Fields are private with explicit getter/setter methods
> - drive() prints full information

```rust
pub struct SportsCar {
    base: Car,
    top_speed: u32,
}
```

> - Contains a Car internally
> - Adds top_speed and boost() to max out performance
> - Overrides drive() to add extra behavior
> - Implements the same Vehicle trait (polymorphic)

### Polymorphism in Action

```rust
pub fn static_dispatch<T: Vehicle>(t: T) {
    println!("From static dispatch:");
    t.drive();
}
```

> - This function accepts any type that implements Vehicle
> - Can be called with Car, SportsCar, or future types like Truck
> - Optional extension: Box<dyn Vehicle> for dynamic dispatch.

---

## How to Run

```bash
cargo build
cargo run
```

Then inside main.rs (not shown here), you can test:

```rust
let mut sc = SportsCar::new("Ferrari", "F8", 2023, 320);
sc.boost();
static_dispatch(sc);
```
