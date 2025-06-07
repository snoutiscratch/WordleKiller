use std::collections::HashMap;

pub struct Entropy<'a> { // entropy has no need for ownership. Everything will only last the lifetime of the instance.
    word_bank: &'a Vec<String>,        // candidate words to consider
    correct: &'a [char; 5],            // green letters
    yellows: &'a Vec<[char; 5]>,       // yellow letters with positional info
    not: &'a Vec<char>,                // letters excluded
}

impl<'a> Entropy<'a> {
    pub fn new(
        word_bank: &'a Vec<String>,
        correct: &'a [char; 5],
        yellows: &'a Vec<[char; 5]>,
        not: &'a Vec<char>
    ) -> Self {
        Self {
            word_bank,
            correct,
            yellows,
            not
        }
    }

    // Simulate guessing every solution word in the word bank and return a pattern of colors.
    // This pattern can be used to figure out how much info can be gained.
    fn simulate(guess: &str, solution:&str) -> String{
        let mut feedback = ['b';5]; // Stores guess data. All black for now.
        let mut used_letters = [false;5]; // Letters used in solution
        let guess_chars: Vec<char> = guess.chars().collect();
        let solution_chars: Vec<char> = solution.chars().collect();

        // Finding greens
        for i in 0..5 {
            if guess_chars[i] == solution_chars[i] {
                feedback[i] = 'g'; // This is part of the solution set
                used_letters[i] = true; // Used this letter
            }
        }

        // Marking yellows
        for i in 0..5 {
            if feedback[i] == 'g' {
                continue; // as this slot has already been marked.
            }

            for j in 0..5 {
                if !used_letters[j] && guess_chars[i] == solution_chars[j] {
                    feedback[i] = 'y';
                    used_letters[j] = true;
                    break;
                }
            }
        }

        feedback.iter().collect()
    }

    // Return the entropy score for a given word... I DON'T UNDERSTAND THIS!! PLS HELP
    pub fn calculate_entropy(&self, guess: &str) -> f64 {
        let mut pattern_counts: HashMap<String, usize> = HashMap::new();   // Counts occurrences for patterns from the simulator
        let total_solutions = self.word_bank.len();                 // Total solutions. Used later for probability calculations
        let mut entropy = 0.0; // Cumulative entropy value

        // Get the simulator pattern for each word in the solution set.
        for solution in self.word_bank.iter() {
            let pattern = Self::simulate(guess, solution);
            *pattern_counts.entry(pattern).or_insert(0) +=1;
        }

        // Finds entropy based on frequency distribution of patterns.
        for &count in pattern_counts.values() {
            let p = count as f64 / total_solutions as f64;
            entropy -= p * p.log2();
        }

        entropy // Where higher values reveal more information about the word
    }
}
