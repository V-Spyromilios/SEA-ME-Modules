# Rust × Slint Phonebook

A **graphical phonebook application** built in 100 % Rust with a Slint UI.
Add, search, and delete contacts in a clean two-tab interface, with data
persisted to **`contacts.json`**.

<div align="center">
  <img src="docs/screenshot_1.png" width="620" alt="Phonebook screenshot 1">
  <img src="docs/screenshot_2.png" width="620" alt="Phonebook screenshot 2">
</div>

---

## Features

| Capability           | Where it lives                       | Notes                                                |
| -------------------- | ------------------------------------ | ---------------------------------------------------- |
| **Add contacts**     | _Add_ tab – `AddTab` component       | Name / phone / email validation & instant persist    |
| **Search live**      | _Search_ tab – `SearchTab` component | Case-insensitive search across all fields            |
| **Delete**           | Trash-icon button in each row        | Updates both UI list & JSON file                     |
| **JSON persistence** | `phonebook.rs`                       | Pretty-printed `contacts.json` for easy hand-editing |
| **MVU-style state**  | `Phonebook` struct + Slint bindings  | UI reacts to state changes via `ModelRc`             |

---
