use std::io;
use std::process;

mod verifier;
use verifier::Verifier;

fn main() {
    println!("Wordle Game");
    loop {
        let mut service = Verifier::new("vero");
        println!("**** Guess what is this a word?");
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

            if service.verify(input) {
                println!("Congratulations, you have guessed the word!");
                break;
            } else {
                println!("{}\n", service.hint());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
}
