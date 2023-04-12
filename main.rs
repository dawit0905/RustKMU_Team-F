use std::io::{self, Write};
use rand::seq::SliceRandom;

fn main() {
    let mut rng = rand::thread_rng();
    let words = vec!["apple", "banana", "orange", "peach", "kiwi", "grape", "lemon"];
    let word = words.choose(&mut rng).unwrap();
    let mut letters: Vec<char> = vec!['_'; word.len()];
    let mut guesses = Vec::new();
    let mut incorrect_guesses = 0;

    println!("Welcome to Hangman!");
    println!("The word has {} letters. You can make 6 incorrect guesses.", word.len());

    loop {
        println!("\n{} incorrect guesses so far.", incorrect_guesses);
        println!("Word: {}", letters.iter().collect::<String>());
        print!("Guess a letter: ");
        io::stdout().flush().unwrap();
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();
        let guess = guess.trim().chars().next().unwrap();
        if guesses.contains(&guess) {
            println!("You already guessed that letter.");
        } else if word.contains(guess) {
            println!("Good guess!");
            for (i, letter) in word.chars().enumerate() {
                if letter == guess {
                    letters[i] = guess;
                }
            }
            if !letters.contains(&'_') {
                println!("You win! The word was {}.", word);
                break;
            }
        } else {
            println!("Sorry, that letter is not in the word.");
            incorrect_guesses += 1;
            if incorrect_guesses == 6 {
                println!("You lose! The word was {}.", word);
                break;
            }
        }
        guesses.push(guess);
    }
}