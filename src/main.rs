use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!"); // This is a macro

    let mut secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess."); // This is a macro

        // This is a mutable variable, and it's bound to new empty string
        let mut guess = String::new();

        io::stdin() // Returns an instance of std::io::Stdin to handle the user input
            // '&' is an indication of a reference and by specifying mut,
            // we say that this 'guess' is a mutable reference.
            // By deafult, all references are immutable, and hence we require 'mut' keyword to make
            // the ref immutable
            .read_line(&mut guess) // This read_line will return 'Result'
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // Return 'num' and store it in 'guess'
            Err(_) => continue, // Continue the loop, ignore the error while parsing
        };

        println!("You guessed: {guess}"); // This is a macro

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Bingo! You win!");

                let mut game_continue = String::new();
                println!("Want to play again? (y/n)");
                io::stdin().read_line(&mut game_continue).expect("Failed to capture user input!");
                if game_continue.trim().to_lowercase() == "n"{
                    println!("Ok Bye!");
                    break;
                } else {
                    secret_number = rand::thread_rng().gen_range(1..=100);
                }
            }
        }
    }
}