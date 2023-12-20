// Guessing Game
//To obtain user input and then print the result as output, we need to bring the io input/output library

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    //generate secret number from 1-100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    //prompt to begin game
    println!("Hello, User, please enter your name to begin the guess game!");

    //creating a mutable variable to store a user name
    let mut name = String::new();

    //stdin command for fetching a user input
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    //printing a user name and prompting user to enter a number for the guess game
    println!("Hello {name}");

    loop {
        println!("T0 begin, enter your guess number");

        //creating a mutable variable to store a user guess
        let mut guess = String::new();

        //stdin command for fetching a user guess
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //convert guess string into an integer
        let guess: u32 = guess
            .trim()
            .parse()
            .expect("You can only use a number for this guess game!");

        //print guessed number
        println!("You guessed {guess}");

        //function that compares guessed number to the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your guess is lower"),
            Ordering::Greater => println!("Your guess is higher"),
            Ordering::Equal => {
                println!("Your guess is correct, you win!!!");
                break;
            }
        }
    }
}
