use reqwest::{get, Error};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
// use serde_json::Value;
use structopt::StructOpt;

#[derive(Debug, Serialize, Deserialize)]
struct WOTD {
    word: String,
    id: u32,
}

#[derive(Debug, StructOpt)]
struct Cli {
    arg: String,
    guess: Option<String>,
}

#[derive(Debug)]
struct GameState {
    word_vec: Vec<char>,
    guessed_vec_copy: Vec<char>,
    guess_count: u32,
}

impl GameState {
    async fn new() -> Result<Self, Error> {
        let new_word = Self::get_new_word().await?;
        let word_vec: Vec<char> = new_word.to_lowercase().chars().collect();
        let guessed_vec_copy: Vec<char> = word_vec.iter().map(|_x| '_').collect();
        println!("Your word is {} characters long.", new_word.len());
        println!("psst, it is: {}", new_word);
        let gamestate = GameState {
            word_vec,
            guessed_vec_copy,
            guess_count: 5,
        };
        Ok(gamestate)
    }

    async fn get_new_word() -> Result<String, Error> {
        let env = fs::read_to_string(".env").expect("Something went wrong reading the file");
        let api_key: Vec<&str> = env.split("api_key=").collect();
        let api_key: String = api_key[1].to_string();
        let url = format!("https://api.wordnik.com/v4/words.json/randomWord?hasDictionaryDef=true&minLength=5&maxLength=-1&api_key={}", api_key);
        let resp = get(&url).await?.text().await?;
        let wotd: WOTD = serde_json::from_str(&resp).unwrap();
        Ok(wotd.word)
    }

    pub fn try_word(&mut self, guess: String) -> bool {
        if guess.len() == 1 {
            let guess_as_char = &guess
                .chars()
                .next()
                .expect("The guess could not be interpreted properly.");

            if self.word_vec.contains(guess_as_char) {
                let guessed_char_indics: Vec<usize> = self
                    .word_vec
                    .iter()
                    .enumerate()
                    .filter_map(|(i, &x)| if x == *guess_as_char { Some(i) } else { None })
                    .collect();

                if guessed_char_indics.len() > 0 {
                    println!("KAZAM! {} correct match!", guessed_char_indics.len());
                    let mut is_complete = 1;
                    let mut spaces_left = 0;
                    self.guessed_vec_copy = self
                        .word_vec
                        .iter()
                        .enumerate()
                        .map(|(i, &x)| {
                            if x == *guess_as_char {
                                x
                            } else if x == self.guessed_vec_copy[i] {
                                self.guessed_vec_copy[i]
                            } else {
                                spaces_left = spaces_left + 1;
                                is_complete = 0;
                                '_'
                            }
                        })
                        .collect();

                    if is_complete == 1 {
                        println!("Congrats! You finished the word! Now nobody needs to die for some reason 🤷");
                        return true;
                    } else {
                        println!(
                            "Good Job! Keep guessing! You have {} spaces left",
                            spaces_left
                        );
                    }
                } else {
                    println!("whoops, could not map {} to a hit.", guess);
                    return false;
                }
            } else {
                self.guess_count = self.guess_count - 1;
                println!(
                    "Nope! \"{}\" is not in the word. You have {} guesses left.",
                    guess_as_char, self.guess_count
                );
                return false;
            }
        }
        if guess.len() > 1 {
            println!("Guess one character at a time.");
        }
        if guess.len() == 0 {
            println!("Guess something ya ding dong.");
        }
        // let guess_output: String = guessed_vec_copy.clone().into_iter().collect::<String>();
        let guess_output: String = self
            .guessed_vec_copy
            .iter()
            .map(|x| format!("{} ", x.to_string()))
            .collect();
        println!("{:?}", guess_output);
        return false;
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut new_game = GameState::new().await?;

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess = guess.trim().to_lowercase();
        let is_complete = new_game.try_word(guess);
        if is_complete {
            break;
        }
    }
    Ok(())
}
