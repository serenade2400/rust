use std::io::{self, Read};  // Import necessary modules from the standard library for input/output
use rand::Rng; // Import the Rng trait from the rand library to generate random numbers
use std::cmp::Ordering; // Import the Ordering enum for comparing values
fn main() {
    println!("Guess the number!");
    // Generate a random number between 1 and 100 (inclusive)
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop { // Start an infinite loop to keep prompting the user until they guess correctly
        println!("Please input your guess:");
        let mut guess = String::new(); // Create a mutable variable to store the user's guess
        io::stdin() // Read the user's input from standard input
            .read_line(&mut guess)
            .expect("Failed to read line");
        // Convert the user's input from a string to an unsigned 32-bit integer
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // If parsing fails, skip to the next iteration of the loop
        };
        println!("You guessed: {}", guess);
        // Compare the user's guess to the secret number using the Ordering enum
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You won!");
                break; // Exit the loop when the correct guess is made
            }
        }
    }}
