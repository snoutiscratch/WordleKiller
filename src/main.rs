use crate::wordle::Solver;
mod wordle;

/* -------------------------------------- */
/* Wordle for dirty cheaters!             */
/* -------------------------------------- */

fn main() {
    let mut s:Solver = Solver::new();
    s.load_words("word_bank.txt");

    loop {
        println!("\n\n---======= :oo =======---");

        println!("Suggested Guesses: {:?}", s.ranked_list());

        // Display progress //
        print!("Correct: ");
        for (_, c) in s.correct.iter().enumerate() {
            if *c == ' ' {
                print!("- ");
            } else {
                print!("{} ", c);
            }
        }
        println!();
        println!("Invalid: {}", s.not.iter().collect::<String>());
        println!("Possible: {}", s.possible.iter().collect::<String>());

        // Main menu //
        println!("--=== DATA ENTRY");
        println!("n) NOT in word");
        println!("p) POSSIBLE letters");
        println!("c) CORRECT position");
        println!("rrr) RESET");

        let choice = wordle::readline(">> ");
        match choice.as_str() {
            "n" => {
                println!("Enter characters NOT in the word:");
                s.add_not();
            }
            "p" => {
                println!("Enter characters POSSIBLE in the word:");
                s.add_possible()
            }
            "c" => {
                println!("Enter the CORRECT positions of letters");
                s.set_correct();
            }
            "rrr" => {
                println!("RESETTING SOLVER!");
                s= Solver::new();
                s.load_words("word_bank.txt");
            }
            _ => println!("NUH UH !!")
        }
    }
}