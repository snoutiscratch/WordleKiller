use std::ascii::AsciiExt;
use std::fs::File;
use std::vec::Vec;
use std::io::{BufRead, BufReader, Write};
use std::str::Chars;
use rustyline;

fn main() {
    println!("Hai you dirty cheater!!");

    let word_bank: Vec<String> = load_words("word_bank.txt");
    let mut words: Vec<String> = word_bank;

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
                print!("{} ", c);
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
                set_correct(&mut correct);
            }
            "4" => {
                solve(&mut words, &mut correct, &mut possible, &mut not);
                println!("{:?}", words);
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

fn set_correct(array: &mut [char; 5]) {
    // Taking input
    let mut rl = rustyline::DefaultEditor::new().unwrap();
    let user_input = match rl.readline(">> ") {
        Ok(line) => line,
        Err(_) => {
            "Line can't be read".to_string()
        }
    };

    let chars: Vec<char> = user_input.chars().collect();
    if chars.len() != 5 {
        println!("Must be 5 characters long. Use spaces for blank or unknown.");
        return;
    }

    for i in 0..5 {
        array[i] = if chars[i] == ' ' { ' ' } else { chars[i].to_ascii_lowercase()};
    }
}

fn solve(words: &mut Vec<String>, correct: &[char; 5], possible: &Vec<char>, not: &Vec<char>) {
    words.retain(|word| {
        let word_arr: [char;5] = word.chars()
            .collect::<Vec<char>>()
            .try_into()
            .expect("Word must be 5 chars");

        // 1) Check correct positions
        for i in 0..5 {
            if correct[i] != ' ' && correct[i] != word_arr[i] {
                return false; // Remove word
            }
        }

        // 2) Check all possible letters present somewhere
        for &letter in possible {
            if !word_arr.contains(&letter.to_ascii_lowercase()) {
                return false; // Remove word
            }
        }

        // 3) Check no forbidden letters
        for &letter in not {
            if word_arr.contains(&letter.to_ascii_lowercase()) {
                return false; // Remove word
            }
        }

        // Word passed all tests — keep it
        true
    });

    println!("Filtered word bank.");
}