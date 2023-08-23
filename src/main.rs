extern crate core;

use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {}", secret_number);
    let mut guess = String::new();
    println!("Please input your guess.");
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    let guess: i32 = guess.parse::<i32>().unwrap();
    println!("Guess the number!");

    println!("You guessed: {guess}");
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
