use rand::Rng;
use std::io::stdin;

fn main() {
    println!("Hello, world! Welcome to number guessing game");
    // computer gets a number
    // guess number
    // tells you too high or too low until its right
    let number = rand::thread_rng().gen_range(1..100);
    let mut tries = 1;
    loop {
        // enter guess
        println!("Enter your guess: ");
        // creating buffer to store guess
        let mut buffer = String::new();
        //read line from console and pass ref to buffer
        // pass to match
        match stdin().read_line(&mut buffer) {
            Ok(_) => {
                // parse into number
                // trim end to remove line break
                // parse into data type
                let parsed = buffer.trim_end().parse::<i64>();
                // test parsed
                match parsed {
                    // test guess
                    Ok(guess) => {
                        if guess < 1 || guess > 100 {
                            println!("your guess is out of range");
                        } else if guess < number {
                            println!("your guess is too low");
                        } else if guess > number {
                            println!("your guess is too high");
                        } else {
                            println!("Correct! It took you {} tries to guess", tries);
                            break;
                        }
                    }
                    Err(e) => {
                        println!("Could not read your input. Please try again. Error: {}", e);
                    }
                }
            }
            Err(_) => continue,
        }
        tries += 1;
    }
}
