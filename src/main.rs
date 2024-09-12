use rand::Rng;
use std::{io, vec};

fn main() {
    let mut rng = rand::thread_rng();
    let n: i32 = rng.gen_range(0..10);

    let multiples: Vec<i32> = vec![50, 100, 150, 200, 250];
    let random_multiple: i32 = multiples[rng.gen_range(0..multiples.len())];

    let hint: i32 = random_multiple * n;

    println!("Welcome to the number guessing game!");
    println!("I have generated a random number between 0 and 9.");
    println!("Try to guess the number!");
    println!("Hint: The number is a multiple of {}", hint);
    
    let mut input = String::new();
    println!("Choose a number:");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let input: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a number");
            return;
        }
    };

    if input == n {
        println!("Congratulations! You guessed the correct number.");
        println!("The correct number was: {}", n);
        println!("The random multiple was: {}", random_multiple);
        return;
    } else {
        println!("Sorry, that's not the correct number. The correct number was: {}", n);
        println!("The random multiple was: {}", random_multiple);
        return;
    }
}