use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Write};
use std::vec::Vec;

pub struct Solver {
    pub words: Vec<String>,
    pub correct: [char; 5],
    pub possible: Vec<char>,
    pub not: Vec<char>,
}

impl Solver {
    // Constructor & Setup //
    pub fn new() -> Self {
        Self {
            words: Vec::new(),
            correct: [' '; 5],
            possible: Vec::new(),
            not: Vec::new(),
        }
    }

    pub fn load_words(&mut self, file: &str) {
        let file = File::open(file).expect("File no loady!!");
        let reader = BufReader::new(file);

        for cur_line in reader.lines() {
            if let Ok(mut line) = cur_line { // Verify line can be read
                line = line.to_lowercase(); // Ensure everything is lowercase
                self.words.push(line); // Save line to vec
            } else {
                println!("Cannot read line!")
            }
        }

        println!("{} words loaded!", self.words.len());

    }

    // Manipulation //
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
    pub fn add_possible(&mut self) {
        Self::add_chars(&mut self.possible);
    }

    pub fn set_correct(&mut self) {
        // Taking input
        let user_input = readline("Correct >> ");

        let chars: Vec<char> = user_input.chars().collect();
        if chars.len() != 5 {
            println!("Must be 5 characters long. Use spaces for blank or unknown.");
            return;
        }

        for i in 0..5 {
            self.correct[i] = if chars[i] == ' ' { ' ' } else { chars[i].to_ascii_lowercase()};
        }
    }

    // Solve //
    pub fn solve(&mut self) {
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

            // 2) Check all possible letters present somewhere
            for letter in self.possible.iter() {
                if !word_arr.contains(&letter.to_ascii_lowercase()) {
                    return false; // Remove word
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

        println!("Filtered word bank.");
    }

    pub fn ranked_list(&mut self) -> Vec<String> {
        let mut output: Vec<String> = Vec::new();
        let mut freq: HashMap<char, usize> = HashMap::new(); // Letter frequency
        let mut scored: Vec<(String, usize)> = Vec::new();

        self.solve(); // Clean up the word bank before additional operations

        // Find letter frequency
        for word in self.words.iter() {
            for c in word.chars() {
                let counter = freq.entry(c).or_insert(0);
                *counter += 1;
            }
        }
        
        // Rank words based on what has the most high-ranking letters
        println!("Ranking word bank");
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
        scored.sort_by(|a,b| b.1.cmp((&a.1)));

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
    println!("{}", prompt);
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