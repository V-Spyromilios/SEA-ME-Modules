# Car Composition in Rust

Here we model an entire car — engine, wheels, brakes, transmission, — using traits, enums, and structs that mirror classic OOP composition.
This repo is a Rust translation of the C++ “Model your own Car” exercise, with stronger type safety, zero-cost abstractions, and ergonomic ownership management.

---

## Rust Concepts

| Concept                     | Where it appears                                      | Why it matters                                                                              |
| --------------------------- | ----------------------------------------------------- | ------------------------------------------------------------------------------------------- |
| **Traits as interfaces**    | `Part`, `Vehicle`                                     | Enforces a common API without inheritance headaches.                                        |
| **Enum-based polymorphism** | `BrakeKind`, `BrakeMaterial`, `WheelType`, `CarModel` | Expresses discrete variants with the compiler guarding exhaustiveness.                      |
| **Smart ownership**         | `Vec<Box<dyn Vehicle>>` in `main`                     | Stores heterogeneous vehicles behind trait objects while keeping heap allocations explicit. |
| **Mod organisation**        | `mod parts;`, nested modules                          | Mirrors package-by-feature layout familiar to iOS/Android devs.                             |

---

## Project Structure

```text
.
├── Cargo.toml
└── src
    ├── main.rs          # demo garage
    ├── part.rs          # `Part` trait
    ├── vehicle.rs       # `Vehicle` trait
    ├── car.rs           # `Car` struct implements `Vehicle`
    └── parts            # each concrete `Part`
        ├── engine.rs
        ├── brake.rs
        ├── transmission.rs
        └── wheel.rs

```

## Build & Run

```bash
cargo run
```

#### Expected terminal snippet:

```bash
--- Garage Slot 1 ---
Starting Custom("Ferrari F8 Tributo")...
From Engine: 710 HP, 3.9L
Wheel: 20" Alloy

```

### Adding Your Own Ride

```rust
use crate::car::CarModel;
use parts::{
    engine::Engine, wheel::{Wheel, WheelType},
    brake::{Brake, BrakeKind, BrakeMaterial},
    transmission::Transmission,
};

let model  = CarModel::Custom("Aston Martin DB12".into());
let engine = Engine::new(671, 4.0);
let wheels = vec![Wheel::new(21, WheelType::Alloy); 4];
let brakes = vec![Brake::new(BrakeKind::Disc, BrakeMaterial::Carbon); 4];
let gearbx = Transmission::new(8, true);

let db12 = Car::new(model, engine, wheels, brakes, gearbx);
garage.push(Box::new(db12));
```
