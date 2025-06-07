use crate::wordle::Solver;

mod wordle;

/* -------------------------------------- */
/* Wordle for dirty cheaters!             */
/* -------------------------------------- */

fn main() {
    println!("Hai you dirty cheater!!");
    let mut s = Solver::new();
    s.load_words("word_bank.txt");

    loop {
        println!("\n\n---======= :oo =======---");

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
        println!("1) NOT in word");
        println!("2) POSSIBLE letters");
        println!("3) CORRECT position");

        let choice = wordle::readline(">> ");
        match choice.as_str() {
            "1" => {
                println!("Enter characters NOT in the word:");
                s.add_not();
            }
            "2" => {
                println!("Enter characters POSSIBLE in the word:");
                s.add_possible()
            }
            "3" => {
                println!("Enter the CORRECT positions of letters");
                s.set_correct();
            }
            "4" => {
                s.solve();
                println!("{:?}", s.words);
            }
            _ => println!("NUH UH !!")
        }
    }
}