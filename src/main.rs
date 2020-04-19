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

async fn get_new_word() -> Result<String, Error> {
    let env = fs::read_to_string(".env").expect("Something went wrong reading the file");
    let api_key: Vec<&str> = env.split("api_key=").collect();
    let api_key: String = api_key[1].to_string();
    let url = format!("https://api.wordnik.com/v4/words.json/randomWord?hasDictionaryDef=true&minLength=5&maxLength=-1&api_key={}", api_key);
    let resp = get(&url).await?.text().await?;
    let wotd: WOTD = serde_json::from_str(&resp).unwrap();
    Ok(wotd.word)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let new_word = get_new_word().await?;
    let word_vec: Vec<char> = new_word.to_lowercase().chars().collect();
    let mut guessed_vec_copy: Vec<char> = word_vec.iter().map(|_x| '_').collect();
    println!("Your word is {} characters long.", new_word.len());
    println!("psst, it is: {}", new_word);
    let mut guess_count = 5;

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess = guess.trim().to_lowercase();
        if guess.len() == 1 {
            let guess_as_char = &guess
                .chars()
                .next()
                .expect("The guess could not be interpreted properly.");

            if word_vec.contains(guess_as_char) {
                let guessed_char_indics: Vec<usize> = word_vec
                    .iter()
                    .enumerate()
                    .filter_map(|(i, &x)| if x == *guess_as_char { Some(i) } else { None })
                    .collect();

                if guessed_char_indics.len() > 0 {
                    println!("KAZAM! {} correct match!", guessed_char_indics.len());
                    let mut is_complete = 1;
                    guessed_vec_copy = word_vec
                        .iter()
                        .enumerate()
                        .map(|(i, &x)| {
                            if x == *guess_as_char {
                                x
                            } else if x == guessed_vec_copy[i] {
                                guessed_vec_copy[i]
                            } else {
                                is_complete = 0;
                                '_'
                            }
                        })
                        .collect();
                    if is_complete == 1 {
                        println!("Congrats! You finished the word! Now nobody needs to die for some reason 🤷");
                        break;
                    }
                } else {
                    println!("whoops, could not map {} to a hit.", guess);
                }
            } else {
                guess_count = guess_count - 1;
                println!(
                    "Nope! \"{}\" is not in the word. You have {} guesses left.",
                    guess_as_char, guess_count
                );
            }
        }
        if guess.len() > 1 {
            println!("Guess one character at a time.");
        }
        // let guess_output: String = guessed_vec_copy.clone().into_iter().collect::<String>();
        let guess_output: String = guessed_vec_copy
            .iter()
            .map(|x| format!("{} ", x.to_string()))
            .collect();
        println!("{:?}", guess_output);
    }

    println!("Ending the game . . . goodbye.");
    Ok(())
}
