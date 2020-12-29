use std::io::stdin;

enum State {
    Locked,
    Failed,
    Unlocked
}

fn main() {
    println!("Hello, world!");
    let code = String::from("1234");
    let mut state = State::Locked;
    let mut entry = String::new();

    loop {
        match state {
            State::Locked => {
                let mut input = String::new();
                // here we are creating a mutable input variable
                // using the &mut pointer, we are passing the mutated variable
                // as an arg for the read_line function
                // stdin read line has 2 states, ok and error
                // the match function will handle both
                match stdin().read_line(&mut input) {
                    Ok(_) => {
                        // appends the input with the most recent char
                        entry.push_str(&input.trim_end())
                    }
                    Err(_) => { continue; }
                }
                if entry == code {
                    state = State::Unlocked;
                    continue;
                }
                // immediate out, test the init char
                if !code.starts_with(&entry) {
                    state = State::Failed;
                }
            }
            State::Unlocked => {
                // exit loop on unlock
                println!("Unlocked");
                return;
            }
            State::Failed => {
                println!("Failed");
                // clears the entry
                entry.clear();
                state = State::Locked;
                continue;
            }
        }
    }
}
