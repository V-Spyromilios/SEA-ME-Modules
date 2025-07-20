# Rust Warm-up Exercise 01: Sports Car (Inheritance in Rust)

This project demonstrates how to model classic Object-Oriented Programming (OOP) inheritance — specifically a `Car` and `SportsCar` — using idiomatic Rust patterns like **composition**, **traits**, and **struct specialization**.

---

## Objective

Simulate classical inheritance:

- `Car` struct with fields: `name`, `speed_kmh`
- `SportsCar` builds on `Car` and adds: `top_speed`
- Demonstrate reusability by calling base behavior from derived logic

> Rust does not have class inheritance. This exercise shows how to represent similar behavior through **composition + traits**.

---

## Running the Code

```bash
cargo build
cargo run
```

## Expected output

```bash
Fiat 500 is flying at 40 km/h!
Car { name: "Fiat 500", speed_kmh: 40 }
Fiat 500 is flying at 40 km/h!
```
