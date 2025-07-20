use std::env;

/*
 * cargo build and then the executable will be target/debug/convert
 * run with ./target/debug/convert up "rust language"
 * or with cargo run down HELLO! (or cd target/debug and ./convert...)
 */

fn main() {
    // args() return arguments with iterator but cannot be indexed.
    let arguments: Vec<String> = env::args().collect(); //consumes the iterator puts everything in collection

    if arguments.len() != 3 {
        eprintln!("Usage: convert <up | down> text"); //prints to io::stderr()
        std::process::exit(1); // returns 1 to the OS
    }

    let command = &arguments[1]; // shared reference - borrows the arg
    let text = &arguments[2];

    match command.as_str() {
        "up" => println!("{}", text.as_str().to_uppercase()
    }
}

// &str is a view into a sequence of UTF-8 bytes
// String is a heap-allocated, growable container
// most string processing functions work on str -> no need for ownership or alloc
