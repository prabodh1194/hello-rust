extern crate core;

use rand::Rng;
use std::io::stdin;

fn main() {
    println!("Guess the number game!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("Enter the number");

    loop {
        let mut guess = String::new();
        stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => panic!("{}", e),
        };
        println!("Guessed number {}", guess);

        if guess == secret_number {
            println!("You win!");
            break;
        } else if guess > secret_number {
            println!("Too high!");
        } else {
            println!("Too low!");
        }
    }
}
