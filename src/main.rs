use std::io;
use std::fs;
use reqwest::{get, Error};
use serde::{Deserialize, Serialize};
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

async fn get_new_word () -> Result<String, Error> {
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
    let mut guess = String::new();
    let mut word_blanks = String::new();
    let mut guessed_copy = String::new();
    for _c in new_word.chars() { 
        word_blanks.push_str("_ ");
    }
    guessed_copy = word_blanks;
    println!("Your word is {} characters long. it is: {}", new_word.len(), new_word);
    while &guess != "^C" {
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        if new_word.contains(&guess) {
            let guessed_char_index = new_word.find(&guess).unwrap();
            word_blanks = String::new();
            for (i, _c) in new_word.chars().enumerate() {
                if i == guessed_char_index {
                    word_blanks.push_str(&format!("{} ", &guess).to_string());
                } else {
                    word_blanks.push_str("_ ");
                }
            }
            guessed_copy = word_blanks;
            println!("Hell yea! You got one. What's your next guess: {}", guessed_copy);
        }
        
    }
    println!("Ending the game . . . goodbye.");
    Ok(())
}
