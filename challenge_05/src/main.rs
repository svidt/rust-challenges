use rand::random;
use std::io;

fn main() {
    let randomNumber: u8 = random();
    // println!("DEBUG: randomNumber = {}", randomNumber);

    let mut stillGuessing: bool = true;

    while stillGuessing {
        let mut guess = String::new();
        println!("Enter your guess: ");
        io::stdin().read_line(&mut guess);

        let number: u8 = guess.trim().parse().unwrap();

        if number > randomNumber {
            println!("Too high, try again!");
        } else if number < randomNumber {
            println!("Too low, try again!");
        } else {
            println!("✨ {} was correct! ✨", number);
            break;
        }
    }
}
