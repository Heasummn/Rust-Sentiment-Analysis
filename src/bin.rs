use std::env;
use dialoguer::{Select, Input, theme::ColorfulTheme};
use glob::{glob, Paths};

// TO USE, RUN:
// $ rustc src/bin.rs
// $ ./bin --flag

use sentiment_analyzer::analysis;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::path::{Path, PathBuf};

fn main() {
    select_file();
    return;

    let arguments: Vec<String> = env::args().collect();

    if arguments.len() < 2 {
        // Using the actual CLI (activates with no command line arguments);
        let integrations = vec!["CSV File Input", "Twitter Data"];
        let input_method = init_cli(integrations);

        
        match input_method {
            0 => println!("Index 0"),
            1 => println!("Index 1"),
            _ => println!("Unseen index!") //Should never happen (new function called for each input format)
        }

        let string = get_input("Test Input");
        println!("{}", string);
        
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
fn read_from_file(filename: &str) -> Vec<sentiment::Analysis>{

    let file = File::open(filename).expect("Error reading file");
    let buf = BufReader::new(file);
    let inputs:Vec<String> = buf.lines() .map(|l| l.expect("Could not parse line")).collect();

    return strings_to_analyses(inputs);    
}

fn strings_to_analyses(inputs: Vec<String>) -> Vec<sentiment::Analysis>{
    let mut to_return:Vec<sentiment::Analysis> = Vec::new();

    for s in inputs{
        let a = analysis::analyze_sentiment(s);
        to_return.push(a);
    }

    return to_return;
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

fn select_file() {

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
        if (i == 0) {
            current_path.pop();
        } else {
            current_path.push(&options[i])
        }
    }

}