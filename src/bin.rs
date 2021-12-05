#[macro_use]
extern crate dotenv_codegen;
extern crate dotenv;


use std::env;
use dialoguer::{Select, Input, theme::ColorfulTheme};
use glob::{glob, Paths};

// TO USE, RUN:
// $ rustc src/bin.rs
// $ ./bin --flag

use sentiment_analyzer::analysis;
use sentiment_analyzer::message::Message;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use chrono::{DateTime, Utc};
use std::time::SystemTime;
use csv::Reader;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    // select_file();
    // return;

    let arguments: Vec<String> = env::args().collect();

    if arguments.len() < 2 {
        // Using the actual CLI (activates with no command line arguments);
        let integrations = vec!["CSV File Input", "Twitter Data"];
        let input_method = init_cli(integrations);


        match input_method {
            0 => {
                println!("Index 0");
                // let string = get_input("Filename");
                let string = select_file();
                // let out = strings_to_analyses(read_from_csv(&string.to_string()));
                let out = strings_to_analyses(read_from_csv(&string));
                analysis::display(&out[0]);
            },
            1 => {
                let string = get_input("User");
                let out = strings_to_analyses(twitter_user_to_messages(string, 10).await);
                analysis::display(&out[0]);
            },
            _ => println!("Unseen index!") //Should never happen (new function called for each input format)
        }

        // let string = get_input("Test Input");
        // println!("{}", string);

    } else {
        // Passed in as arguments, and not using the CLI
        let input_format = arguments[1].to_lowercase(); //Second flag in the series (input format)
        let split: Vec<&str> = input_format.split("=").collect(); //Second flag, but split in key-value pair

        if split.len() != 2 {
            // More temporary error handling - invalid flag passed in
            println!("Invalid input! (Case B)");
            // initialize cli again here?
        }

        match split[0] {
            "--t" | "--twitter" => println!("Twitter"),
            "--f" | "--file" => println!("File"),
            _ => {
                // Last potential case of error handling - flag passed in DNE
                println!("Invalid input! (Case C)")
            }
        }
    }
}

/*
Assumptions I made;
    - input will be read from a txt file. if it's a different format, this should be pretty easy to adjust
*/
fn read_from_file(filename: &str) -> Vec<analysis::AnalysisResult> {

    let file = File::open(filename).expect("Error reading file");
    let buf = BufReader::new(file);
    let inputs:Vec<Message> = buf.lines() .map(|l| Message::new(l.expect("Could not parse line"), DateTime::from(SystemTime::now()))).collect();

    return strings_to_analyses(inputs);
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

fn strings_to_analyses(inputs: Vec<Message>) -> Vec<analysis::AnalysisResult>{
    let mut to_return:Vec<analysis::AnalysisResult> = Vec::new();

    for s in inputs{
        let a = analysis::analyze_sentiment(s);
        to_return.push(a);
    }

    return to_return;
}

async fn twitter_user_to_messages(handle: String, page_size: i32) -> Vec<Message> {
    // read from .env file
    dotenv().ok();
    let con_token = egg_mode::KeyPair::new(dotenv!("API_KEY", "API_KEY is not set!"), dotenv!("API_SECRET", "API_SECRET is not set!"));
    let access_token = egg_mode::KeyPair::new(dotenv!("ACCESS_TOKEN", "ACCESS_TOKEN is not set!"), dotenv!("ACCESS_SECRET", "ACCESS_SECRET is not set!"));
    let token = egg_mode::Token::Access {
        consumer: con_token,
        access: access_token,
    };

    let user_id : egg_mode::user::UserID = handle.into();
    // let user = egg_mode::user::show(user, &token).await.unwrap();
    let timeline = egg_mode::tweet::user_timeline(user_id, true, true, &token).with_page_size(page_size);
    let (timeline, feed) = timeline.start().await.unwrap();
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

fn select_file() -> String{

    let mut current_path = PathBuf::from(std::env::current_dir().unwrap());
    
    while true {
        println!("current path: {}", current_path.display());
        env::set_current_dir(&current_path.display().to_string());
        let p : glob::Paths = glob("*").unwrap();
        let mut options : Vec<String> = Vec::new();
        
        if (current_path.display().to_string() != "/") {
            options.push("..".to_string());
        } else {
            // THROW ERROR
        }
        
        for i in p {
            let s = i.unwrap().display().to_string();
            options.push(s);
        }
        
        let i = init_cli(options.iter().map(|x| x as &str).collect());
        if i == 0 && options[0] == ".." {
            current_path.pop();
        } else {
            current_path.push(&options[i]);
        }
        
        if !current_path.is_dir() {
            let x = current_path.extension();
            match x {
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
    return "".to_string();

}
