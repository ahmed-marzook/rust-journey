use std::cmp::Ordering;
use std::io;

use rand::prelude::*;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::rng().random_range(1..=100); // using a random number generating library rand whcih we added as a dependency inc cargo.toml file we can generate random numbers

    loop {
        println!("Please input your guess."); // println! is a rust macro as specified by the exclamation mark

        let mut guess = String::new(); // create a mutable variable with type string

        io::stdin() // using the io library to read user input and read line resturns a result which we then do some error handling with expect
            .read_line(&mut guess) // what ever is typed by the user is read and then stored in the mutable variable guess
            .expect("Failed to read line"); // error handling

        let guess: u32 = match guess.trim().parse() {
            // converting the guess mutable variable to guess which is shadowing it this is not mutable so its an integer or type u32
            Ok(num) => num, // using the match expression on the parse as it retruns a result and we can handle any errors
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            // using the match ordering to comapre the number and output and then break the loop
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
