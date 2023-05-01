extern crate rand;

use rand::{thread_rng, Rng};
use std::{cmp::Ordering, io};

fn main() {
    println!("Lets start the guessing game");

    let secret_number = thread_rng().gen_range(1, 101);

    loop {
        println!("Please enter your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Unable to read input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!!"),
            Ordering::Greater => println!("Too big!!"),
            Ordering::Equal => {
                println!("You win!!");
                break;
            }
        };
    }
}
