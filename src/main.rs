
use std::io;
fn main() { 
    println!("Guess the number!");

    println!("Please input your guess.");
    let mut guess = String::new(); //variables are immutable by default in rust, mut crate a nutable var
    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
