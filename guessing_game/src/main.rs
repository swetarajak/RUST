use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number");

    let secret_number = rand::rng().random_range(1..=100);

    // println!("The secret numbet is : {secret_number}");
    loop {
        println!("Enter your guess:");
        let mut guess = String::new(); // mut -> guess variable is mutable 

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        println!("Your guess : {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Small!"),
            Ordering::Greater => println!("Big!"),
            Ordering::Equal => {
                println!("You Win!!");
                break;
            }
        }
    }
}
