use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Write};
use std::vec::Vec;

use crate::entropy::Entropy;

pub struct Solver {
    pub words: Vec<String>,         // Possible words
    pub correct: [char; 5],         // Letters in the correct spot
    pub yellows: Vec<[char; 5]>,    // Letters that are in the word, but not in this spot
    pub not: Vec<char>,             // Letters not in the word
    pub file: String,
}

impl Solver {
    // Constructor & Setup //
    pub fn new() -> Self {
        Self {
            words: Vec::new(),
            correct: [' '; 5],
            yellows: Vec::new(),
            not: Vec::new(),
            file: "allowed.txt".to_string(),
        }
    }

    pub fn load_words(&mut self, file_name: &str) {
        let file = File::open(file_name).expect("File no loady!!");
        let reader = BufReader::new(file);

        // Multifile support
        self.file = file_name.to_string();
        self.words = Vec::new();

        for cur_line in reader.lines() {
            if let Ok(mut line) = cur_line {        // Verify line can be read
                line = line.to_lowercase();                // Ensure everything is lowercase
                self.words.push(line);                     // Save line to vec
            } else {
                println!("Cannot read line!")
            }
        }

        println!("{} words loaded!", self.words.len());
    }

    pub fn swap_file(&mut self) {
        if self.file == "allowed.txt" {
            println!("Using answers.txt");
            self.load_words("answers.txt");
        } else {
            println!("Using allowed.txt");
            self.load_words("allowed.txt");
        }
    }

    // DISPLAY //
    pub fn display_overview(&mut self) {
        println!("\n\n---======= Overview =======---");

        // Show suggestions
        self.filter(); // Cleans up solution set before running intensive ratings
        let frequency_list: Vec<String> = self.frequency_rank();

        println!("Top Frequency based guesses: {:?}", frequency_list);
        println!("Top Entropy based guesses: {:?}", self.entropy_rank(frequency_list));

        // Hints
        println!("Invalid: {}", self.not.iter().collect::<String>());
        println!("\nYellows:\n{}", self.display_yellow());
        print!("Correct: ");
        for (_, c) in self.correct.iter().enumerate() {
            if *c == ' ' {
                print!("- ");
            } else {
                print!("{} ", c);
            }
        }
    }

    fn display_yellow(&self) -> String {
        let mut out = String::new();

        for row in &self.yellows {
            out.push_str("[ ");

            // Placing letters and placeholders
            for (_, &c) in row.iter().enumerate() {
                if c == ' ' {
                    out.push('-');
                } else {
                    out.push(c)
                }
            }

            out.push_str(" ]\n");
        }

        out
    }

    // Manipulation //
    fn input_position(prompt: &str) -> Vec<char> {
        loop {
            let user_input = readline(&prompt);
            let chars: Vec<char> = user_input.chars().collect();

            if chars.len() != 5 {
                println!("Must be 5 characters long. Use spaces for blank or unknown.");
            } else {
                return chars
            }
        }
    }

    fn add_chars(list: &mut Vec<char>) {
        let user_input = readline(">> ");

        // Parse into chars and push to a list
        for c in user_input.chars() {
            let c = c.to_ascii_lowercase(); // No caps
            if !list.contains(&c) { // Duplicate prevention
                list.push(c);
            }
        }
    }
    pub fn add_not(&mut self) {
        Self::add_chars(&mut self.not);
    }

    pub fn add_yellows(&mut self) {
        let chars: Vec<char> = Self::input_position("Yellows >>");
        let mut yellow_arr = [' '; 5];
        for i in 0..5 {
            yellow_arr[i] = if chars[i] == ' ' { ' ' } else { chars[i].to_ascii_lowercase()};
        }

        self.yellows.push(yellow_arr);
    }

    pub fn set_correct(&mut self) {
        let chars: Vec<char> = Self::input_position("Correct >>");
        for i in 0..5 {
            self.correct[i] = if chars[i] == ' ' { ' ' } else { chars[i].to_ascii_lowercase()};
        }
    }

    // Solve //
    pub fn filter(&mut self) {
        self.words.retain(|word| {
            let word_arr: [char;5] = word.chars()
                .collect::<Vec<char>>()
                .try_into()
                .expect("Word must be 5 chars");

            // 1) Check correct positions
            for i in 0..5 {
                if self.correct[i] != ' ' && self.correct[i] != word_arr[i] {
                    return false; // Remove word
                }
            }

            // 2) Yellows check
            if let Some(yellow_arr) = self.yellows.last() { // If last row exists
                for i in 0..5 {
                    let letter = yellow_arr[i];
                    
                    // No letter
                    if letter == ' ' {
                        continue;
                    }
                    
                    // Position check
                    if word_arr[i] == letter {
                        return false
                    }
                    
                    // Not in word
                    if !word_arr.contains(&letter) {
                        return false;
                    }
                }
            }

            // 3) Check no forbidden letters
            for letter in self.not.iter() {
                if word_arr.contains(&letter.to_ascii_lowercase()) {
                    return false; // Remove word
                }
            }

            // Word passed all tests — keep it
            true
        });

        println!("Word bank filtered.");
    }

    pub fn frequency_rank(&mut self) -> Vec<String> {
        let mut output: Vec<String> = Vec::new();
        let mut freq: HashMap<char, usize> = HashMap::new(); // Letter frequency
        let mut scored: Vec<(String, usize)> = Vec::new();

        // Find letter frequency
        for word in self.words.iter() {
            for c in word.chars() {
                let counter = freq.entry(c).or_insert(0);
                *counter += 1;
            }
        }
        
        // Rank words based on what has the most high-ranking letters
        for word in &self.words {
            let mut score = 0;
            let mut seen = HashSet::new();
            
            for c in word.chars() {
                if !seen.contains(&c) {
                    seen.insert(c);
                    if let Some(&count) = freq.get(&c) {
                        score += count;
                    }
                }
            }
            scored.push((word.clone(), score))
        }

        // Sorting the ranks
        scored.sort_by(|a,b| b.1.cmp(&a.1));

        // TOP 10 THINGS TO DO BEFORE YOU DIE!
        let limit = if scored.len() < 100 { scored.len() } else { 100 };
        for i in 0..limit {
            output.push(scored[i].0.clone());
        }
        output
    }

    // Calculates word rankings based on entropy, which returns better guesses.
    // COMPUTATIONALLY INTENSIVE: Use filter() to reduce the solution set's size.
    pub fn entropy_rank(&mut self, sublist: Vec<String>) -> Vec<String> {
        let mut output: Vec<String> = Vec::new();
        let mut scored: Vec<(String, f64)> = Vec::with_capacity(self.words.len()); // Store word and entropy score
        let e = Entropy::new(&sublist);

        // Calculate entropy for every word in solution set
        for i in 0..self.words.len() {
            let word = &self.words[i];
            let entropy = e.calculate_entropy(word);
            scored.push((word.clone(), entropy));
        }

        // Sort entropy by highest to lowest
        scored.sort_by(|a, b| {
            b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal)
        });

        // TOP 10 THINGS TO DO BEFORE YOU DIE!
        let limit = if scored.len() < 10 { scored.len() } else { 10 };
        for i in 0..limit {
            output.push(scored[i].0.clone());
        }
        output
    }
}

// Helpers //
pub fn readline(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap(); // Make sure prompt shows immediately

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Don't trim end — just strip newline
    if input.ends_with('\n') {
        input.pop(); // removes `\n`
        if input.ends_with('\r') { input.pop(); } // handles Windows `\r\n`
    }
    
    return input;
}