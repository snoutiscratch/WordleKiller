use crate::wordle::Solver;
mod wordle;
mod entropy;
/* -------------------------------------- */
/* Wordle for dirty cheaters!             */
/* -------------------------------------- */

fn main() {
    let mut s:Solver = Solver::new();
    s.swap_file();
    
    loop {
        // Display progress //
        s.display_overview();

        // Main menu //
        println!("\n--=== DATA ENTRY ===--");
        println!("n) NOT in word");
        println!("y) POSSIBLE letters (Yellows)");
        println!("c) CORRECT position");
        println!("ss) Switch Source File");
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
            "ss" => {
                println!("Switching source files. This should be done late game.");
                s.swap_file();
            }
            "as" => {
                println!("All Solutions: {:?}", s.words);
            }
            "rr" => {
                println!("RESETTING SOLVER!");
                s= Solver::new();
                s.load_words("answers.txt");
            }
            _ => println!("NUH UH !!")
        }
    }
}