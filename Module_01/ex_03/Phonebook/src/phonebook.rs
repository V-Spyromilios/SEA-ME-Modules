use crate::contact::Contact;

/*
 *  contacts: Vec::new(), is idiomatic for:
 *
 *  let contacts = Vec::new();
 *  Phonebook { contacts }
 */

pub struct Phonebook {
    contacts: Vec<Contact>,
}

impl Phonebook {
    pub fn new() -> Self {
        Self {
            contacts: Vec::new(),
        }
    }

    // check for duplicate phone number, then add
    pub fn add(&mut self, contact: Contact) {
        let mut contact_exists: bool = false;

        for existing_contact in &self.contacts {
            if contact.phone == existing_contact.phone {
                contact_exists = true
            }
        }
        if contact_exists {
            println!(
                "Contact with number: '{}' already in the phonebook.",
                contact.phone
            );
        } else {
            println!("Contact added to the phonebook.");
            contact.display();
            self.contacts.push(contact); //push() takes ownership
        }
    }

    pub fn search(&self) {
        // list contacts with index
    }

    pub fn remove(&mut self, by: &str) {
        // remove by index or phone
    }

    // filter and list
    pub fn list_bookmarked(&self) {
        for existing_contact in &self.contacts {
            if existing_contact.bookmarked {
                existing_contact.display();
            }
        }
    }
}
