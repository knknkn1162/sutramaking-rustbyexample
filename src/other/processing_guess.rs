extern crate rand;

use std::io;
use std::cmp::Ordering;
use self::rand::Rng;

pub fn test() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        println!("{:?}", guess.trim().parse::<u32>());
        let guess = guess.trim().parse::<u32>().unwrap_or(1);

        println!("You guessed : {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}