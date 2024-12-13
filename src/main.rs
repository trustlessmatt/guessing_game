use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // use rand crate to generate random number

    // debug: println!("The secret number is {secret_number}");

    loop {
        println!("Please input your guess:");

        let mut guess = String::new(); // initialize a mutable string variable for the guess

        io::stdin() // standard library :: input
            .read_line(&mut guess) // locks this handle and reads a line of input, appending it to the specified buffer.
            .expect("Failed to read line"); // error handling for Result type

        let guess: u32 = match guess.trim().parse() { // parse() returns Result type, which we access
            Ok(num) => num, // if parse returns Ok, access the value and pass it to the matcher
            Err(_) => continue, // if parse returns Err, continue will repeat the loop again
        }; 

        println!("You guessed: {}", guess); // supposedly can also do "You guessed: {guess}"

        match guess.cmp(&secret_number) { // compare guess to the secret number
            // cmp::Ordering has 3 variants:
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    
}