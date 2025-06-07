use crate::wordle::Solver;
mod wordle;
mod entropy;
/* -------------------------------------- */
/* Wordle for dirty cheaters!             */
/* -------------------------------------- */

fn main() {
    let mut s:Solver = Solver::new();
    s.load_words("word_bank.txt");

    loop {
        // Display progress //
        s.display_overview();

        // Main menu //
        println!("\n--=== DATA ENTRY ===--");
        println!("n) NOT in word");
        println!("y) POSSIBLE letters (Yellows)");
        println!("c) CORRECT position");
        println!("as) ALL SOLUTIONS");
        println!("rr) RESET");

        let choice = wordle::readline(">> ");
        match choice.as_str() {
            "n" => {
                println!("Enter characters NOT in the word:");
                s.add_not();
            }
            "y" => {
                println!("Enter characters POSSIBLE in the word:");
                s.add_yellows()
            }
            "c" => {
                println!("Enter the CORRECT positions of letters");
                s.set_correct();
            }
            "as" => {
                s.filter();
                println!("All Solutions: {:?}", s.words);
            }
            "rr" => {
                println!("RESETTING SOLVER!");
                s= Solver::new();
                s.load_words("word_bank.txt");
            }
            _ => println!("NUH UH !!")
        }
    }
}