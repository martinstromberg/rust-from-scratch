pub mod hangman;
pub mod blackjack;

use crate::hangman::play_hangman;

fn main() {
    play_hangman("lines_of_words.txt");
}
