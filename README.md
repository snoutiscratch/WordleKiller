# WordleKiller
A tool for people who are bad at wordle 😼

---

## Sample Run
```shell
> ./WordleKiller
2315 words loaded!


---======= :oo =======---
Filtered word bank.
Ranking word bank
Suggested Guesses: ["alert", "alter", "later", "arose", "irate", "stare", "arise", "raise", "learn", "renal"]
Correct: - - - - -
Invalid:
Possible:
--=== DATA ENTRY
n) NOT in word
p) POSSIBLE letters
c) CORRECT position
rrr) RESET
>> n
Enter characters NOT in the word:
>> altr
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
### Word List Source
The word list included with this solver was sourced from [cfreshman/wordle-answers-alphabetical.txt](https://gist.github.com/cfreshman/a03ef2cba789d8cf00c08f767e0fad7b)

### Narrowing
For the best performance, the word list is narrowed as much as possible every cycle.

The order is as follows: Correct -> Possible letters -> Letters not in word.

### Suggested Guesses
Calculated by figuring out the most common letters in a list of possible words given hints.

The more common letters a word has, the higher its rank on the suggested guess list.

---

<small>this is the first useful-ish program I made in rust, pls don't be mean</small>
