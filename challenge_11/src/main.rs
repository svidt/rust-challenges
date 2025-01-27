use std::io;
use rand::prelude::*;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("I'm thinking of a number between 1 and 100...");
    println!("Guess the number:");

    loop {
        let mut guess = String::new();

        match io::stdin().read_line(&mut guess) {
            Ok(_) => (),
            Err(error) => {
                println!("Error reading the input: {}", error);
                continue;
            }
        }
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        if guess > secret_number {
            println!("\n{} is too high! Guess lower:", guess);
        } else if guess < secret_number {
            println!("\n{} is too low! Guess higher:", guess);
        } else {
            println!("\nYou got it! The secret number was {}.", secret_number);
            break;
        }
    }
}