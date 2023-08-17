use std::io;
use std::process;

mod verifier;
use verifier::{GuessResult, Verifier};

mod words;
use words::Words;

fn main() {
    println!("Wordle Game");
    let mut words = Words::new();

    loop {
        let mut service = Verifier::new(&words.sample().unwrap());
        println!("Please enter your guess word. It should be 4 characters long");
        loop {
            let mut buf = String::new();
            io::stdin().read_line(&mut buf).unwrap();
            let input = buf.trim();

            if input == "exit" {
                process::exit(0);
            }
            if input == "restart" {
                break;
            }
            if input.len() != 4 {
                println!("Please enter a 4-characters word");
                continue;
            }

            match service.verify(input) {
                GuessResult::Guessed => {
                    println!("Congratulations, you have guessed the word!");
                    break;
                }
                GuessResult::Missed { hint } => {
                    println!("{hint}\n");
                }
            }
        }
    }
}
