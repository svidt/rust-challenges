use std::env;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    if env::args().len() <= 2 {
        println!("Please provide at least two arguments. <file path> <search name>");
        return;
    }

    let path_arg = env::args().nth(1).unwrap();
    let name_arg = env::args().nth(2).unwrap();

    println!("Search for '{}' in file: {}", name_arg, path_arg);

    // Read the entire file content into a string
    let mut file = match File::open(&path_arg) {
        Ok(file) => file,
        Err(e) => {
            println!("Error opening file: {}", e);
            return;
        }
    };

    let mut contents = String::new();
    if let Err(e) = file.read_to_string(&mut contents) {
        println!("Error reading file: {}", e);
        return;
    }

    // Search for the name in the file contents
    if contents.contains(&name_arg) {
        println!("Found '{}' in the file.", name_arg);
    } else {
        println!("'{}' was not found in the file.", name_arg)
    }
}
