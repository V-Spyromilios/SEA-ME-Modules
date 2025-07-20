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
            println!("");
            println!("Contact added to the phonebook:");
            contact.display();
            self.contacts.push(contact); //push() takes ownership
        }
    }

    pub fn get_contact_counter(&self) -> usize {
        self.contacts.len()
    }

    pub fn get_mutable_contact(&mut self, i: usize) -> Option<&mut Contact> {
        // let mut indexed_contacts = self.get_indexed_contacts();
        self.contacts.get_mut(i)
    }

    pub fn get_indexed_contacts(&self) -> Vec<(usize, &Contact)> {
        self.contacts.iter().enumerate().collect()
    }

    // remove by index or phone
    pub fn remove(&mut self, by: &str) -> bool {
        // Try by phone number
        if let Some(pos) = self.contacts.iter().position(|c| c.phone == by) {
            let removed = self.contacts.remove(pos);
            println!("✅ Removed contact: '{}'", removed.name);
            return true;
        }

        // Try by index (if it's not a phone match)
        if let Ok(index) = by.parse::<usize>() {
            if index < self.contacts.len() {
                let removed = self.contacts.remove(index);
                println!("✅ Removed contact: '{}' at index {}", removed.name, index);
                return true;
            } else {
                println!("⚠️ No contact at position: {}", index);
                return false;
            }
        }

        println!(
            "Input '{}' is neither a valid phone number nor a valid index.",
            by
        );
        false
    }

    // filter and list
    pub fn list_bookmarked(&self) {
        for existing_contact in &self.contacts {
            if existing_contact.is_bookmarked {
                existing_contact.display();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_sample_contact(index: usize) -> Contact {
        Contact {
            name: format!("Person{}", index),
            phone: format!("555000{}", index),
            nickname: format!("P{}", index),
            is_bookmarked: false,
        }
    }

    #[test]
    fn test_add_contact() {
        let mut pb = Phonebook::new();
        let contact = create_sample_contact(1);
        pb.add(contact);
        assert_eq!(pb.get_contact_counter(), 1);
    }

    #[test]
    fn test_add_duplicate_phone() {
        let mut pb = Phonebook::new();
        let contact1 = create_sample_contact(1);
        let contact2 = Contact {
            name: String::from("Other"),
            phone: contact1.phone.clone(),
            nickname: String::from("OtherNick"),
            is_bookmarked: false,
        };
        pb.add(contact1);
        pb.add(contact2); // should not be added
        assert_eq!(pb.get_contact_counter(), 1);
    }

    #[test]
    fn test_remove_by_phone() {
        let mut pb = Phonebook::new();
        let contact = create_sample_contact(0);
        let phone = contact.phone.clone();
        pb.add(contact);
        let removed = pb.remove(&phone);
        assert!(removed);
        assert_eq!(pb.get_contact_counter(), 0);
    }

    #[test]
    fn test_remove_by_index() {
        let mut pb = Phonebook::new();
        pb.add(create_sample_contact(0));
        let removed = pb.remove("0"); // stringified index
        assert!(removed);
        assert_eq!(pb.get_contact_counter(), 0);
    }

    #[test]
    fn test_remove_invalid_input() {
        let mut pb = Phonebook::new();
        pb.add(create_sample_contact(0));
        let removed = pb.remove("not_a_number");
        assert!(!removed);
        assert_eq!(pb.get_contact_counter(), 1);
    }

    #[test]
    fn test_get_mutable_contact() {
        let mut pb = Phonebook::new();
        pb.add(create_sample_contact(1));
        if let Some(contact) = pb.get_mutable_contact(0) {
            contact.is_bookmarked = true;
        }
        assert!(pb.contacts[0].is_bookmarked);
    }

    #[test]
    fn test_remove_from_empty_phonebook() {
        let mut pb = Phonebook::new();
        let removed = pb.remove("0");
        assert!(!removed);
    }

    #[test]
    fn test_remove_out_of_bounds_index() {
        let mut pb = Phonebook::new();
        pb.add(create_sample_contact(0));
        let removed = pb.remove("99");
        assert!(!removed);
        assert_eq!(pb.get_contact_counter(), 1);
    }

    #[test]
    fn test_bookmark_listing() {
        let mut pb = Phonebook::new();
        let mut c1 = create_sample_contact(1);
        let c2 = create_sample_contact(2);
        c1.is_bookmarked = true;
        pb.add(c1);
        pb.add(c2);
        let bookmarked: Vec<_> = pb.contacts.iter().filter(|c| c.is_bookmarked).collect();
        assert_eq!(bookmarked.len(), 1);
    }
}
