slint::include_modules!();
mod contact;
mod phonebook;

use contact::Contact;
use phonebook::Phonebook;
use slint::{ModelRc, VecModel};

use std::cell::RefCell;
use std::rc::Rc;

fn main() -> Result<(), slint::PlatformError> {
    let ui = MainWindow::new()?;

    //println!("📁 Current working dir: {:?}", std::env::current_dir());

    let phonebook = Rc::new(RefCell::new(Phonebook::load_from_file("contacts.json")));

    {
        let data: Vec<ContactData> = phonebook
            .borrow()
            .get_contacts()
            .iter()
            .map(|c| ContactData {
                name: c.name.clone().into(),
                phone: c.phone.clone().into(),
                email: c.email.clone().into(),
            })
            .collect();

        // ui.invoke_load_contacts(ModelRc::new(VecModel::from(data)));
        ui.set_contacts(ModelRc::new(VecModel::from(data)));
    }

    {
        let pb = phonebook.clone();
        let ui_handle = ui.as_weak();

        ui.on_add_contact(move |name, phone, email| {
            println!("on_add_contact: {}, {}, {}", name, phone, email);
            pb.borrow_mut().add(Contact {
                name: name.trim().to_string(),
                phone: phone.trim().to_string(),
                email: email.trim().to_string(),
            });

            // Update UI
            if let Some(ui) = ui_handle.upgrade() {
                let data: Vec<ContactData> = pb
                    .borrow()
                    .get_contacts()
                    .iter()
                    .map(|c| ContactData {
                        name: c.name.clone().into(),
                        phone: c.phone.clone().into(),
                        email: c.email.clone().into(),
                    })
                    .collect();

                ui.set_contacts(ModelRc::new(VecModel::from(data)));

                pb.borrow().save_to_file("contacts.json");
            }
        });
    }

    {
        let pb = phonebook.clone();
        let ui_handle = ui.as_weak();

        ui.on_remove_contact(move |name| {
            pb.borrow_mut().remove(&name);

            // Refresh UI
            if let Some(ui) = ui_handle.upgrade() {
                let data: Vec<ContactData> = pb
                    .borrow()
                    .get_contacts()
                    .iter()
                    .map(|c| ContactData {
                        name: c.name.clone().into(),
                        phone: c.phone.clone().into(),
                        email: c.email.clone().into(),
                    })
                    .collect();

                ui.set_contacts(ModelRc::new(VecModel::from(data)));
            }

            pb.borrow().save_to_file("contacts.json");
        });
    }

    {
        let pb = phonebook.clone();
        ui.on_save_contacts(move |_contacts| {
            let all: Vec<Contact> = pb.borrow().get_contacts().to_vec();
            Phonebook { contacts: all }.save_to_file("contacts.json");
        });
    }

    {
        let ui_handle = ui.as_weak();
        ui.on_load_contacts(move |data| {
            if let Some(ui) = ui_handle.upgrade() {
                ui.invoke_load_contacts(data);
            }
        });
    }

    ui.on_search_contacts(move |query| {
        println!("on_search_contacts: {}", query.as_str());
        let pb = phonebook.clone();
        let results = pb.borrow().search(&query);
        let data: Vec<_> = results
            .into_iter()
            .map(|c| ContactData {
                name: c.name.into(),
                phone: c.phone.into(),
                email: c.email.into(),
            })
            .collect();

        ModelRc::new(VecModel::from(data))
    });

    ui.run()
}

/*

 When to Use ModelRc
   Use Vec<T> only inside your logic
   (in your structs and pure Rust functions).
   Use ModelRc<T> whenever you interact with Slint
   callbacks that pass or receive [...] array types.
*/
