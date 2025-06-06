use std::fs::File;
use std::vec::Vec;
use std::io::{BufRead, BufReader};

/* -------------------------------------- */
/* Wordle for dirty cheaters!             */
/* -------------------------------------- */

fn main() {
    println!("Hai you dirty cheater!!");
    
    let word_bank: Vec<String> = load_words("word_bank.txt");
    let possible_words: Vec<String>;

    let not: Vec<char>; // Letters not in the word
    let possible: Vec<char>; // Letters that are in the word
    let correct: [char;5] = [' ', ' ', ' ', ' ', ' ']; // Letters in the correct place
    
}

fn load_words(file: &str) -> Vec<String> {
    let file = File::open(file).expect("File no loady!!");
    let mut words: Vec<String> = Vec::new();

    let reader = BufReader::new(file);

    for cur_line in reader.lines() {
        if let Ok(mut line) = cur_line { // Verify line can be read
            line = line.to_lowercase(); // Ensure everything is lowercase
            words.push(line); // Save line to vec
        } else {
            println!("Cannot read line!")
        }
    }

    println!("{} words loaded!", words.len());
    return words;
}
