extern crate core;

use core::num::dec2flt::number;
use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn type_name_of_val<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {}", secret_number);
    let mut guess = String::new();
    println!("Guess the number!");
    println!("Please input your guess.");
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
    // println!("{}", std::any::type_name::<type_name_of_val(&guess)>());
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
