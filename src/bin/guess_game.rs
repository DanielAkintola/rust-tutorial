#![allow(unused)]

use std::io;
use rand::Rng;

fn main() {
    println!("Guess the Number!!!");
    println!("");
    println!("Enter your guess: ");

    let secret_number = rand::thread_rng().gen_range(0, 100);
    
    let mut guess: String = String::new(); 
    io::stdin().read_line(&mut guess);

    println!("{guess}");

    let guessed_int:i32 = guess.trim().parse().expect("not a valid number");

    if guessed_int == secret_number {
        println!("whao you are correct");
        println!("secret number is {secret_number}");
    } else {
        println!("so sorry you are wrong the secret number is {secret_number}");
    }    
}