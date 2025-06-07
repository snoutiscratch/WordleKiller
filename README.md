# WordleKiller
A tool for people who are bad at wordle 😼

---

## Sample Run
```shell
---======= Overview =======---
Word bank filtered.
Ranking filtered on letter frequency...
Frequency based guesses: ["alert", "alter", "later", "arose", "irate", "stare", "arise", "raise", "learn", "renal"]
Entropy based guesses: ["raise", "slate", "crate", "irate", "trace", "arise", "stare", "snare", "arose", "least"]
Invalid: 

Yellows:

Correct: - - - - - 
--=== DATA ENTRY ===--
n) NOT in word
y) POSSIBLE letters (Yellows)
c) CORRECT position
as) ALL SOLUTIONS
rr) RESET
>> y
Enter characters POSSIBLE in the word:
Yellows >>r    


---======= Overview =======---
Word bank filtered.
Ranking filtered on letter frequency...
Frequency based guesses: ["arose", "irate", "adore", "stare", "opera", "cater", "crate", "trace", "arise", "trade"]
Entropy based guesses: ["trace", "crate", "irate", "trade", "arose", "arise", "tread", "crane", "trice", "stare"]
Invalid: 

Yellows:
[ r---- ]

Correct: - - - - - 
--=== DATA ENTRY ===--
n) NOT in word
y) POSSIBLE letters (Yellows)
c) CORRECT position
as) ALL SOLUTIONS
rr) RESET
>> c
Enter the CORRECT positions of letters
Correct >>  i  
```

## Usage
### Setup
Download or compile an executable. Place the executable alongside the `word_bank.txt` file. You may bring your own `word_bank.txt`, just make sure it's named exactly that.
```tree
.
└── WordleKiller
    ├── WordleKiller
    └── word_bank.txt
```

### Run
Run the executable in the command line or your system's terminal by choosing the directory the app is located in and running the program as shown:
```shell
cd ~/Downloads/WordleKiller/
./WordleKiller
```

## How calculation works
The solution set is narrowed every time the user inputs hints about the word. Eventually this list will be narrowed down to one word.

### Basic Narrowing
For the best performance, the word list is narrowed as much as possible every time new information is added. This is important as entropy is computationally heavy.

The culling order is as follows: Correct -> Possible letters -> Letters not in word. Once this culling is completed, the word list will be handed off to more intensive computions (like frequency and entropy).

### Frequency Suggestions
Calculated by figuring out the most common letters in a list of possible words given hints.
The more common letters a word has, the higher its rank on the suggested guess list.
### Entropy Based Suggestions
Calculated using entropy, which ranks words based on how much information a word would give the player. Words higher up on this list will give the player more information about the word.

### Word List Source
The word list included with this solver was sourced from [cfreshman/wordle-answers-alphabetical.txt](https://gist.github.com/cfreshman/a03ef2cba789d8cf00c08f767e0fad7b)
This solver doesn't rank very high...

---

<small>this is the first useful-ish program I made in Rust, pls don't be mean</small>
