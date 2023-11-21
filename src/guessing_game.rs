use rand::Rng;
use std::io::stdin;

pub fn guess_the_number() {
    let target = rand::thread_rng().gen_range(1..100);

    println!("Guess the number I'm thinking of!");
    let num_guesses = guessing_round(target, 1);
    if num_guesses == -1 {
        println!("Invalid input");
        return;
    }

    println!("You took {} attempts to guess the number", num_guesses);
}

fn guessing_round(target: i32, attempts: i32) -> i32 {
    match read_number_input() {
        Some(guess) => {
            if guess == target {
                attempts
            } else {
                if guess > target {
                    println!("Try a lower guess!")
                } else {
                    println!("Try a higher guess!")
                }

                guessing_round(target, attempts + 1)
            }
        }
        None => -1
    }
}

fn read_number_input() -> Option<i32> {
    let mut input = String::new();

    match stdin().read_line(&mut input) {
        Ok(_) => {
            match input.trim().parse::<i32>() {
                Ok(num) => Some(num),
                Err(_) => None
            }
        }
        Err(_) => None
    }
}
