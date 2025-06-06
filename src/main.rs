use std::fs::File;
use std::vec::Vec;
use std::io::{BufRead, BufReader, Write};

use rustyline;

fn main() {
    println!("Hai you dirty cheater!!");

    let word_bank: Vec<String> = load_words("word_bank.txt");
    let possible_words: Vec<String>;

    let mut not: Vec<char> = Vec::new(); // Letters not in the word
    let mut possible: Vec<char> = Vec::new(); // Letters that are in the word
    let mut correct: [char;5] = [' ', ' ', ' ', ' ', ' ']; // Letters in the correct place

    let mut rl = rustyline::DefaultEditor::new().unwrap();

    /* -------------------------------------- */
    /* Wordle for dirty cheaters!             */
    /* -------------------------------------- */

    loop {
        println!();

        // Display progress //
        print!("Correct: ");
        for(i, c) in correct.iter().enumerate() {
            if *c == ' ' {
                print!("- "); // Empty spaces
            } else {
                print!("{}", c);
            }
        }
        println!();

        println!("Invalid: {}", not.iter().collect::<String>());
        println!("Possible: {}", possible.iter().collect::<String>());


        // Main menu //
        println!("------- DATA ENTRY! -------");
        println!("1) NOT in word");
        println!("2) POSSIBLE letters");
        println!("3) CORRECT position");

        let choice = match rl.readline(">> ") {
            Ok(line) => line,
            Err(_) => {
                println!("Error reading line!");
                continue;
            }
        };

        match choice.as_str() {
            "1" => {
                println!("Enter characters NOT in the word:");
                input_chars(&mut not);
            }
            "2" => {
                println!("Enter characters POSSIBLE in the word:");
                input_chars(&mut possible);
            }
            "3" => {
                println!("Enter the CORRECT positions of letters");
                
            }
            _ => println!("NUH UH !!")
        }

    }
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

fn input_chars(list: &mut Vec<char>) {
    // Taking input
    let mut rl = rustyline::DefaultEditor::new().unwrap();
    let user_input = match rl.readline(">> ") {
        Ok(line) => line,
        Err(_) => {
            "Line can't be read".to_string()
        }
    };

    // Parse into chars and push to list
    for char in user_input.chars() {
        list.push(char);
    }
}
