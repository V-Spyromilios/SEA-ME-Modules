mod contact;
mod phonebook;

// use contact::Contact;
use colored::*;
use figlet_rs::FIGfont;
use phonebook::Phonebook;
use std::io::{self, Write};

use crate::contact::Contact;

// ADD SEARCH REMOVE BOOKMARK EXIT
fn main() {
    let mut phonebook = Phonebook::new();
    print_welcome();

    loop {
        print!("Enter Command (Or EXIT):  ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let command = input.trim();
        match command.to_uppercase().as_str() {
            "EXIT" => {
                println!("{}", "Exiting Phonebook.".bold().yellow());
                break;
            }
            "SEARCH" => {
                search_phonebook(&mut phonebook);
            }
            "ADD" => add_new_contact(&mut phonebook),
            "REMOVE" => remove_by_index_or_phone(&mut phonebook),
            _ => {
                println!("Uknown Command: {}. Try again!", command.bold().yellow());
            }
        }
    }
}

fn remove_by_index_or_phone(pb: &mut Phonebook) {
    if pb.get_contact_counter() == 0 {
        println!("{}", "PhoneBook is empty.".bold().yellow());
        return;
    }
    let indexed = pb.get_indexed_contacts();
    println!("PhoneBook contacts:");
    for (index, contact) in &indexed {
        println!(
            "{} {}  ::  {} {}  ::  {} {}",
            "👤 Name:".bold().cyan(),
            contact.name.bold().white(),
            "📇 Index:".bold().magenta(),
            index.to_string().bold().white(),
            "📞 Phone:".bold().green(),
            contact.phone.bold().yellow()
        );
    }
    let mut choise = String::new();
    loop {
        print!("Enter Index or Number of Contact to Delete (or BACK to return):  ");
        io::stdout().flush().unwrap();
        choise.clear();
        io::stdin().read_line(&mut choise).unwrap();
        choise = choise.trim().to_string();
        if choise.is_empty() {
            break;
        }
        if choise == "BACK" || choise == "back" || choise == "Back" {
            break;
        }
        let success = pb.remove(&choise);
        if success {
            break;
        }
    }
}

fn search_phonebook(pb: &mut Phonebook) {
    if pb.get_contact_counter() == 0 {
        println!("PhoneBook is empty.");
        return;
    }

    let mut selected_index = String::new();

    let indexed = pb.get_indexed_contacts();
    println!("PhoneBook contacts:");
    for (index, contact) in &indexed {
        println!(
            "{} {}  {} {}",
            "👤 Name:".bold().cyan(),
            contact.name.bold().cyan(),
            "📇 Index:".bold().magenta(),
            index.to_string().bold().magenta()
        );
    }

    loop {
        print!("Enter Index of Contact to display (or BACK to return):  ");
        io::stdout().flush().unwrap();
        selected_index.clear();
        io::stdin().read_line(&mut selected_index).unwrap();
        selected_index = selected_index.trim().to_string();
        if selected_index.is_empty() {
            break;
        }
        if selected_index == "BACK" || selected_index == "back" || selected_index == "Back" {
            break;
        }
        let actual_index = match selected_index.parse::<usize>() {
            Ok(value) => value,
            Err(error) => {
                println!("{} {}", "Error:".bold().red(), error.to_string().red());
                return;
            }
        };

        if let Some(mut_contact) = pb.get_mutable_contact(actual_index) {
            loop {
                print!("Should '{}' be bookmarked? (Y/N):  ", mut_contact.name);
                io::stdout().flush().unwrap();
                let mut bool_input = String::new();

                io::stdin().read_line(&mut bool_input).unwrap();
                bool_input = bool_input.trim().to_string().to_lowercase();

                if bool_input.as_str() == "y" {
                    mut_contact.is_bookmarked = true;
                    mut_contact.display();
                    break;
                } else if bool_input.as_str() == "n" {
                    println!("OK. Will not saved as Bookmarked.");
                    break;
                } else {
                    println!("Invalid choice. Use 'Y' for Yes or 'N' for No.")
                }
            }
        };
    }
}

/*
*  let mut contact_pair = indexed[value];
let (_, &mut contact) = contact_pair;
contact.display();
let mut reply = String::new();
print!("Do you want to bookmark? (Y/N):  ");
io::stdout().flush().unwrap();
io::stdin().read_line(&mut reply).unwrap();
reply = reply.trim().to_string().to_lowercase();
if reply.as_str() == "y" {
    contact.is_bookmarked = true;
}
*/

fn add_new_contact(pb: &mut Phonebook) {
    let mut name = String::new();
    let mut phone = String::new();
    let mut nick = String::new();
    let mut boolean_input: bool = false;

    loop {
        print!("Enter Name:  ");
        io::stdout().flush().unwrap();
        name.clear();
        io::stdin().read_line(&mut name).unwrap();
        name = name.trim().to_string();
        if !name.is_empty() {
            break;
        } else {
            println!("Name cannot be empty.");
        }
    }

    loop {
        print!("Enter Phone Number:  ");
        io::stdout().flush().unwrap();
        phone.clear();
        io::stdin().read_line(&mut phone).unwrap();
        phone = phone.trim().to_string();
        if !phone.is_empty() {
            break;
        } else {
            println!("Phone Number cannot be empty.");
        }
    }

    loop {
        print!("Enter Nickname:  ");
        io::stdout().flush().unwrap();
        nick.clear();
        io::stdin().read_line(&mut nick).unwrap();
        nick = nick.trim().to_string();
        if !nick.is_empty() {
            break;
        } else {
            println!("NickName cannot be empty.");
        }
    }

    loop {
        print!("Should '{}' be bookmarked? (Y/N):  ", name);
        io::stdout().flush().unwrap();
        let mut bool_input = String::new();

        io::stdin().read_line(&mut bool_input).unwrap();
        bool_input = bool_input.trim().to_string();
        if bool_input.as_str() == "Y" || bool_input.as_str() == "y" {
            boolean_input = true;
            break;
        } else if bool_input.as_str() == "N" || bool_input.as_str() == "n" {
            break;
        } else {
            println!("Invalid choice. Use 'Y' for Yes or 'N' for No.")
        }
    }

    pb.add(Contact {
        name: name,
        phone: phone,
        nickname: nick,
        is_bookmarked: boolean_input,
    });
}

fn print_welcome() {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("PhoneBook");
    match figure {
        Some(ref fig) => println!("{}", fig),
        None => eprintln!("Error generating ASCII art"),
    }

    println!(
        "{}",
        "📘 PHONEBOOK COMMANDS:  ADD  SEARCH  REMOVE  BOOKMARK"
            .bold()
            .white()
    );
}
