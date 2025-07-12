use crate::contact::Contact;
use std::fs::File;

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
        let mut contents = String::new();
        use std::io::Read;
        let mut f = File::open(path).unwrap();
        f.read_to_string(&mut contents).unwrap();
        println!("📄 File contents: {}", contents);

        match File::open(path) {
            Ok(file) => match serde_json::from_reader(file) {
                Ok(contacts) => Phonebook { contacts },
                Err(e) => {
                    eprintln!("Failed to deserialize contacts: {}", e);
                    Phonebook {
                        contacts: Vec::new(),
                    }
                }
            },
            Err(e) => {
                eprintln!("Failed to open contacts file: {}", e);
                Phonebook {
                    contacts: Vec::new(),
                }
            }
        }
    }
}
