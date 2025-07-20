# Rust Warm-up Exercise 03: Terminal Phonebook

Part of my SEA:ME warm-up exercises — this project recreates the kind of utility software people once ran on character-based interfaces. It's a feature-rich **terminal-based phonebook** implemented in idiomatic Rust with modular design, testing, and user interactivity.

---

## Description

A nostalgic dive into how people used computers before GUIs — no graphics, no mouse, just text. This project brings that era back to life with a robust terminal-based phonebook written in Rust.

### Supported Commands

- `ADD` — Add a new contact (name, phone number, nickname, bookmark)
- `SEARCH` — List contacts by index and display full details
- `REMOVE` — Remove a contact by index or phone number
- `BOOKMARK` — List all bookmarked contacts _(available internally)_
- `EXIT` — Quit the application

---

## How to Run

```bash
cargo build
cargo run
```

## Follow the terminal prompts:

```bash
Enter Command (Or EXIT): ADD
Enter Name: Alice
Enter Phone Number: 5550001
Enter Nickname: Ally
Should 'Alice' be bookmarked? (Y/N): Y

Enter Command (Or EXIT): SEARCH
Enter Index of Contact to display (or BACK to return): 0
Should 'Alice' be bookmarked? (Y/N): N

Enter Command (Or EXIT): REMOVE
Enter Index or Number of Contact to Delete (or BACK to return): 0

Enter Command (Or EXIT): EXIT
```
