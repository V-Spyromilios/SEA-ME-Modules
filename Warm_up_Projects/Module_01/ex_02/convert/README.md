# Rust Warm-up Exercise 00: CLI Text Converter

This is part of a series of warm-up projects to build foundational Rust fluency, with a focus on command-line tools and structuring idiomatic Rust applications. This first task simulates programming in the early days — without GUIs — where only terminal input/output was possible.

## Project Description

The challenge:
Build a basic CLI (Command-Line Interface) application that simulates a text-based utility on an old black-and-white terminal.

### Features

- Takes a **command** (`up` or `down`) and a **string** as arguments
- Converts the input string:
  - `up` → uppercase
  - `down` → lowercase
- Prints the converted result to `stdout`
- Handles incorrect usage with a helpful message to `stderr`

---

## How to Build and Run

```bash

# Run using cargo with arguments
cargo run -- up "rust language"

# Or run the built binary directly
./target/debug/convert down "HELLO!"
```

Expected output:

```bash
RUST LANGUAGE
```
