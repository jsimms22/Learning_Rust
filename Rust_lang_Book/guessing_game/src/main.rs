use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::{self, Colorize}; 

fn main() {
    println!("Guess a number");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("Your secret number is: {}", secret_number);

    loop  {
        println!("Please input a guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guess: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small.".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            },
            Ordering::Greater => println!("{}", "Too big.".red()),
        }
    }
}
