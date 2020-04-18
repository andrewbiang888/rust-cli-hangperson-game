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
    println!("Your word is {} characters long.", new_word.len());
    println!("psst, it is: {}", new_word);
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
                .expect("guess could not be interpreted properly");
            let word_vec: Vec<char> = new_word.to_lowercase().chars().collect();
            let guessed_vec_copy: Vec<char> = word_vec.iter().map(|_x| '_').collect();
            if word_vec.contains(guess_as_char) {
                println!("Nice! {} is in the word! What's your next guess?", guess);
                let guessed_char_indics: Vec<usize> = word_vec
                    .iter()
                    .enumerate()
                    .filter_map(|(i, x)| if x == guess_as_char { Some(i) } else { None })
                    // .clone()
                    .collect();
                if guessed_char_indics.len() > 0 {
                    print!("nice: {}", guessed_char_indics.len());
                }
                print!("Match at index: {:?}, {:?}", guessed_char_indics, word_vec);
                let guessed_vec_copy: Vec<char> = word_vec
                    .iter()
                    .enumerate()
                    .map(|(_, x)| if x == guess_as_char { x.clone() } else { '_' })
                    .collect();
            }
        }
        if guess.len() > 1 {
            println!("Guess one character at a time.");
        }
        // word_vec.iter().map(|char| if char. )
        // println!("guessin {}, {}", guess.trim(), new_word.contains(guess.trim()));
        // if new_word.contains(guess.trim()) {
        //     let guessed_char_index = new_word.find(guess.trim()).unwrap();
        //     println!("nice, {}", guessed_char_index);
        //     word_blanks = String::from(guessed_copy);
        //     for (i, c) in new_word.chars().enumerate() {
        //         println!("working? {}, {}", i, c);

        //         if i == guessed_char_index {
        //             word_blanks.push_str(&format!("{} ", guess.trim()).to_string());
        //         } else {
        //             word_blanks.push_str("_ ");
        //         }
        //     }
        //     guessed_copy = word_blanks;
        //     println!(
        //         "Hell yea! You got one. What's your next guess: {}",
        //         guessed_copy
        //     );
        // };
    }

    println!("Ending the game . . . goodbye.");
    Ok(())
}
