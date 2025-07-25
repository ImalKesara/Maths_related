use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guessing Game !!!!");

    let secret_number = rand::thread_rng().gen_range(1..50);
    loop {
        let mut guess = String::new();
        println!("Enter you guess / number ");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        println!("you guessed value {guess}");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("enter number");
                continue;
            }

        };

        println!("secret number is {secret_number}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you won the game");
                break;
            }
        }
    }
}
