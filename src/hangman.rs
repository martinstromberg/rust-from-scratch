use std::{fs::File, io::Read};
use std::io::{stdin, stdout, self, Write};

use rand::Rng;


pub fn play_hangman(file_path: &'static str) {
    println!("Let's play hangman!");

    loop {
        if run_hangman_round(file_path) {
            println!("A winrar is you!");
        } else {
            println!("Sadge");
        }

        print!("Play again? (y/N) ");
        let _ = stdout().flush();

        let ch = read_char();
        if ch != 'y' {
            break;
        }
    }
}

fn run_hangman_round(file_path: &'static str) -> bool {
    let word = get_word_list(file_path)
        .map(select_word);

    word.map(|w| play_hangman_round(w, 0, 0)).unwrap_or(false)
}

fn play_hangman_round(word: String, tracker: i32, attempts: i32) -> bool {
    print_round(word.clone(), tracker, attempts);

    print!("Enter your guess: ");
    let _ = io::stdout().flush();

    let guess = read_char();

    let updated_tracker = word
        .char_indices()
        .filter(|x| x.1 == guess)
        .map(|x| x.0)
        .fold(tracker, set_index_unlocked);
    let expected = i32::pow(2, word.len().try_into().unwrap()) - 1;

    if expected == updated_tracker {
        true
    } else if attempts == 7 {
        println!("The word was: {}", word);
        false
    } else {
        let cur_attempts = if updated_tracker == tracker { attempts + 1 } else { attempts };

        play_hangman_round(word, updated_tracker, cur_attempts)
    }
}

fn print_round(word: String, tracker: i32, attempts: i32) -> () {
    println!("You have {} attempt(s) left", 8 - attempts);

    let chars: Vec<String> = (0..word.len())
        .map(|i| {
            if is_index_set(tracker, i) {
                word.chars().nth(i).unwrap().to_string()
            } else {
                "_".to_string()
            }
        })
        .collect();
        
    let str: String = chars.join(" ");
    println!("{}", str);
}

fn is_index_set(tracker: i32, index: usize) -> bool {
    if index >= 32 {
        return false
    }

    let mask = 1 << index;
    (tracker & mask) != 0
}

fn set_index_unlocked(tracker: i32, index: usize) -> i32 {
    if index >= 32 {
        return tracker
    }

    let mask = 1 << index;
    tracker | mask
}

fn read_char() -> char {
    let mut input = String::new();

    match stdin().read_line(&mut input) {
        Ok(_) => if let Some(c) = input.to_lowercase().chars().next() { c } else { '\0' },
        Err(_) => '\0'
    }
}

fn select_word(words: Vec<String>) -> String {
    let mut rng = rand::thread_rng();
    let ix = rng.gen_range(0..words.len());

    words[ix].to_lowercase().clone()
}

fn read_lines_from_file(mut file: File) -> Vec<String> {
    let mut file_content = String::new();
    
    let read_res = file.read_to_string(&mut file_content);

    match read_res {
        Ok(n) => {
            if n < 1 {
                return vec!()
            }

            return file_content
                .lines()
                .map(String::from)
                .collect();
        }
        Err(_) => vec!()
    }
}

fn get_word_list(file_path: &'static str) -> Option<Vec<String>> {
    let file = File::open(file_path);
    let lines_res = file.map(read_lines_from_file);

    match lines_res {
        Ok(lines) => if lines.len() == 0 { None } else { Some(lines) },
        Err(_) => None
    }
}

