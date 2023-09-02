use rand::Rng;
use std::process;
use std::io::{stdin,stdout,Write};

fn main() {
    let mut s=String::new();
    let mut rng = rand::thread_rng();
    let death = rng.gen_range(1..6);
    let mut numbers = vec![];
    println!("Russian Roulette! Pick a Number between 1 and 6 ^^");

    loop {
        if numbers.len() < 5 {
            let _ = stdout().flush();
            stdin().read_line(&mut s).expect("Did not enter a correct string");
            if let Some('\n')=s.chars().next_back() {
                s.pop();
            }
            if let Some('\r')=s.chars().next_back() {
                s.pop();
            }

            let result = s.parse::<i32>();
            s.clear();
            match result {
                Ok(number) => {
                    if !(1..=6).contains(&number) {
                        println!("What kind of mag does your revolver have?");
                        continue;
                    }

                    if numbers.contains(&number) {
                        println!("You already tried that one, pick another one.");
                        continue;
                    }

                    if number == death {
                        process::exit(0);
                    } else {
                        println!("Correct! :D Pick another one...");
                        numbers.push(number);
                        continue;
                    }
                }
                Err(e) => {
                    println!("Not a Number... Pick a real number ({})", e);
                    continue;
                }
            }
        } else { // Survived
            println!("Congratulations! You won <3");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read input");
            break;
        }
    }
}