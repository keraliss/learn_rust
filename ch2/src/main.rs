use colored::Colorize;
use rand::Rng;
use std::cmp::Ordering;
use std::io;
use colored::*;

fn main() {
    println!("Guessing Game");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Your secret  : {}", secret_number);

    loop {
        println!("Please input your guess");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You've guessed : {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}","Too Small".red()),
            Ordering::Greater => println!("{}","Too Big".red()),
            Ordering::Equal => {
                println!("{}","Buzzinga".green());
                break;
            }
        }
    }
}
