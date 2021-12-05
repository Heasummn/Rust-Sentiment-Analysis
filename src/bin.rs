#[macro_use]
extern crate dotenv_codegen;
extern crate dotenv;

use std::env;
use dialoguer::{Select, Input, theme::ColorfulTheme};
use glob::glob;
use std::path::PathBuf;
use std::str::FromStr;
use chrono::{DateTime, Utc};
use csv::Reader;
use dotenv::dotenv;
use sentiment_analyzer::map_reduce;
use sentiment_analyzer::message::Message;

#[tokio::main]
async fn main() {
    // Using the actual CLI (activates with no command line arguments);
    let integrations = vec!["CSV File Input", "Twitter Data"];
    let input_method = init_cli(integrations);

    match input_method {
        // CSV File Input
        0 => {
            let string = select_file();
            let out = map_reduce::map_reduce_messages_to_analyses(read_from_csv(&string.to_string())); 
            for a in out {
                println!("{}: {}", a.time.to_rfc2822(), a.result.score)
            }
        },
        // Twitter Data
        1 => {
            let string = get_input("User");
            let out = map_reduce::map_reduce_messages_to_analyses(twitter_user_to_messages(string, 10).await); 
            for a in out {
                println!("{}: {}", a.time.to_rfc2822(), a.result.score)
            }
        },
        _ => println!("Unseen index!") //Should never happen (new function called for each input format)
    }

}

/*
Assumptions:
    - input will be a CSV file with format text, timestamp
*/

fn read_from_csv(filename: &str) -> Vec<Message> {
    let rdr = Reader::from_path(filename).expect("Error reading file");
    let inputs : Vec<Message> = rdr.into_records().map(|row| {
        Message::new(row.as_ref().unwrap()[0].to_string(), DateTime::<Utc>::from_str(&row.unwrap()[1]).unwrap())
    }).collect();

    return inputs;
}

async fn twitter_user_to_messages(handle: String, page_size: i32) -> Vec<Message> {
    // read from .env file: need to run cargo clean if compiled with bad credentials 
    dotenv().ok();

    // Authenticate API
    let con_token = egg_mode::KeyPair::new(dotenv!("API_KEY", "API_KEY is not set!"), dotenv!("API_SECRET", "API_SECRET is not set!"));
    let access_token = egg_mode::KeyPair::new(dotenv!("ACCESS_TOKEN", "ACCESS_TOKEN is not set!"), dotenv!("ACCESS_SECRET", "ACCESS_SECRET is not set!"));
    let token = egg_mode::Token::Access {
        consumer: con_token,
        access: access_token,
    };

    // Get user's timeline
    let user_id : egg_mode::user::UserID = handle.into();
    let timeline = egg_mode::tweet::user_timeline(user_id, true, true, &token).with_page_size(page_size);
    let (_, feed) = timeline.start().await.unwrap();
    
    // iterate over timeline and convert to custom Message format
    let mut ret = Vec::new();
    for tweet in feed.iter() {
        ret.push(Message::new(tweet.text.to_string(), DateTime::<Utc>::from(tweet.created_at)));
    }

    return ret;
}

// Initialize CLI, with the different options. Return choice index
fn init_cli(items: Vec<&str>) -> usize  {
    let selection: usize = Select::with_theme(&ColorfulTheme::default())
        .items(&items)
        .default(0)
        .interact()
        .unwrap();
    return selection;
}

// use the CLI to get a string input
fn get_input(prompt: &str) -> String {
    let input : String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt.to_string())
        .with_initial_text("")
        .interact_text()
        .unwrap();
    return input;
}

// creates a CLI environment to navigate file system and select file
fn select_file() -> String{

    let mut current_path = PathBuf::from(std::env::current_dir().unwrap());
    
    loop {
        println!("current path: {}", current_path.display());
        env::set_current_dir(&current_path.display().to_string()).unwrap();
        let p : glob::Paths = glob("*").unwrap();
        let mut options : Vec<String> = Vec::new();
        
        // add outer directory to options if not at root
        if current_path.display().to_string() != "/" {
            options.push("..".to_string());
        }

        // fill options with all files in current directory
        for i in p {
            let s = i.unwrap().display().to_string();
            options.push(s);
        }
        
        let i = init_cli(options.iter().map(|x| x as &str).collect());
        // checking if user is going to outer directory
        if i == 0 && options[0] == ".." {
            current_path.pop();
        } else {
            current_path.push(&options[i]);
        }
        
        if !current_path.is_dir() {
            let x = current_path.extension();
            match x {
                // check if file is CSV
                // could be extended to take in an iterable and check against those extensions
                Some(e) => {
                    if e == "csv" {
                        return current_path.display().to_string();
                    } else {
                        current_path.pop();   
                    }
                },
                None => {current_path.pop();}
            }
        }

    }
}
