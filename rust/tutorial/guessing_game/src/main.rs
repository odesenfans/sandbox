extern crate rand;

use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_num = rand::thread_rng().gen_range(1, 101);
    let mut nb_attempts: u32 = 0;

    loop {
        println!("Please input your guess:");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        nb_attempts += 1;
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! ({} attempts)", nb_attempts);
                break;
            }
        }
    }
}
