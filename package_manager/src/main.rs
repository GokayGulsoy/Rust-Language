use std::io;
extern crate rand;
// importing external crate
use rand::random;

// gets an input from user
fn get_guess() -> u8 {
    loop {
        println!("Input guess");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("could not read from stdin");

        match guess.trim().parse::<u8>() {
               Ok(v) => return v,
               Err(e) => println!("could not understand input {}",e)
        }
    }
}

// function to handle the guess
fn handle_guess(guess: u8,correct: u8) -> bool {
    if guess < correct {
        println!("Too low");
        false
    }

    else if guess > correct {
        println!("Too high");
        false
    }

    else {
        println!("You got it ..");
        true
    }
}

// driver code for guessing game
fn main() {
    println!("Welcome to number guessing game");

    let correct: u8 = random();
    loop {
        let guess: u8 = get_guess();
            if handle_guess(guess,correct) {
                break;
            }
    }
}