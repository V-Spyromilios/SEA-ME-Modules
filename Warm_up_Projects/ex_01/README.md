# Rust × Slint “Thread Race”

A tiny **multithreaded racing-game demo** written in pure Rust with a Slint UI.
Three coloured 'cars' (red, green, blue) sprint across the window; each
car runs in its **own OS thread**, racing to the finish line at position 100.

> Exercise goal: practise _threads, shared state (Arc + Mutex), and UI updates
> from worker threads_—the Rust analogue of the original Qt/C++
> multi-thread-racing assignment.

---

## Project structure

```shell
.
├── Cargo.toml            # deps + slint-build config
├── build.rs              # slint::include_modules!() glue
└── src
    ├── main.rs           # wires UI ↔ threads
    ├── car.rs            # Car struct + thread helper
    ├── racetrack.rs      # RaceTrack struct
    └── slint
        └── upWindow.slint     # MainWindow

```

## ✨ Who is Who

| Topic                       | Where it lives                   | Why it matters                                             |
| --------------------------- | -------------------------------- | ---------------------------------------------------------- |
| **Car model**               | `src/car.rs`                     | Holds `position`, `speed`, plus `move_forward()`.          |
| **Race track**              | `src/racetrack.rs`               | Provides `is_finished()` for fair end-of-race detection.   |
| **Thread spawning**         | `spawn_car_thread()` in `car.rs` | Each thread loops: move → sleep 100 ms → report.           |
| **Cross-thread UI updates** | `slint::invoke_from_event_loop`  | Safely mutates Slint properties from worker threads.       |
| **Arc + Mutex**             | Main & `spawn_car_thread`        | Enables shared, mutable `Car` structs without data races.  |
| **Animated UI**             | `app.slint`                      | Simple rectangles whose `x` property is bound to position. |

---

## How it works

1. **Thread per car**
   `spawn_car_thread()` receives an `Arc<Mutex<Car>>`. Inside an infinite loop it:
   - Locks the car and calls `move_forward()`
   - Checks `RaceTrack::is_finished`
   - Pushes the fresh position back to the UI via `slint::invoke_from_event_loop`
   - Sleeps **100 ms** to simulate time

2. **Safe cross-thread UI updates**
   Slint requires all UI mutations to happen on its own event-loop thread—hence the `invoke_from_event_loop` closure.

3. **Restart logic**
   Clicking the button resets each car’s `position` to `0` and spawns three brand-new threads.

#### Click “Start Race” (subsequent clicks restart).
