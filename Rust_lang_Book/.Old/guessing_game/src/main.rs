use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number. ");

    let secret_number = rand::thread_rng().gen_range(1..101);

    //println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //this is a weird way to convert types coming from C/C++
        //trim() removes whitespace
        //parse() converts guess to identified type specified to the left of = to a result
        //expect() is a result handling error function; if err than it will reply with string given
        let guess: u32 = match guess.trim().parse() {
            //removed error(string) to allow result to enum the input. Will either continue the loop 
            //or go to next iteration and prompt a new guess
            Ok(num) => num,
            //_ is a "catch-all" value
            Err(_) => continue,
        };

        println!("You guess: {}", guess);    

        //works similar to switch statements from C/C++
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}