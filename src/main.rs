extern crate core;

use std::io::stdin;

fn main() {
    println!("Guess the number game!");

    println!("Enter the number");

    let mut guess = String::new();

    stdin().read_line(&mut guess)
        .expect("Failed to read line");

    let guess: f32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(e) => panic!("{}", e),
    };

    println!("Guessed number {}", guess);
}
