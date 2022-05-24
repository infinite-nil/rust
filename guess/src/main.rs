use colored::{self, Colorize};
use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number");
    println!("Please input your guess");

    let secret = rand::thread_rng().gen_range(1, 101);

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("{}", "Too small".red()),
            Ordering::Greater => println!("{}", "Too big".red()),
            Ordering::Equal => {
                println!("{}", "You win!".blue());
                break;
            }
        }
    }
}
