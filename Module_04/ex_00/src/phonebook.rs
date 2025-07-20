use crate::contact::Contact;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;

#[derive(Clone)]
pub struct Phonebook {
    pub contacts: Vec<Contact>,
}
impl Phonebook {
    pub fn new() -> Self {
        Phonebook {
            contacts: Vec::new(),
        }
    }

    pub fn add(&mut self, contact: Contact) {
        self.contacts.push(contact);
    }

    pub fn get_contacts(&self) -> &Vec<Contact> {
        &self.contacts
    }

    pub fn remove(&mut self, name: &str) {
        self.contacts.retain(|c| c.name.trim() != name.trim());
    }

    pub fn search(&self, query: &str) -> Vec<Contact> {
        let query_lower = query.trim().to_lowercase();

        if query_lower.is_empty() {
            return self.contacts.clone();
        }

        let results: Vec<Contact> = self
            .contacts
            .iter()
            .filter(|c| {
                c.name.to_lowercase().contains(&query_lower)
                    || c.phone.to_lowercase().contains(&query_lower)
                    || c.email.to_lowercase().contains(&query_lower)
            })
            .cloned()
            .collect();

        if results.is_empty() {
            self.contacts.clone()
        } else {
            results
        }
    }

    pub fn save_to_file(&self, path: &str) {
        let file = File::create(path).unwrap();
        serde_json::to_writer_pretty(file, &self.contacts).unwrap();
    }

    pub fn load_from_file(path: &str) -> Self {
        // ① Open **read-write**, creating the file when missing
        let mut file = match OpenOptions::new()
            .read(true)
            .write(true)
            .create(true) // <-- the magic bit
            .open(path)
        {
            Ok(f) => f,
            Err(e) => {
                eprintln!("❌ Cannot open or create `{}`: {}", path, e);
                return Self {
                    contacts: Vec::new(),
                };
            }
        };

        // ② Slurp the whole file into a String
        let mut contents = String::new();
        if let Err(e) = file.read_to_string(&mut contents) {
            eprintln!("❌ Failed to read `{}`: {}", path, e);
            return Self {
                contacts: Vec::new(),
            };
        }

        // ③ Empty file? => start with an empty phonebook
        if contents.trim().is_empty() {
            return Self {
                contacts: Vec::new(),
            };
        }

        // ④ Parse JSON; fall back to empty Vec on error
        match serde_json::from_str::<Vec<Contact>>(&contents) {
            Ok(contacts) => Self { contacts },
            Err(e) => {
                eprintln!("❌ JSON parse error in `{}`: {}", path, e);
                Self {
                    contacts: Vec::new(),
                }
            }
        }
    }
}
