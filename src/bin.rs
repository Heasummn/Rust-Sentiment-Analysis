use std::env;
use dialoguer::{Select, Input, theme::ColorfulTheme};

// TO USE, RUN:
// $ rustc src/bin.rs
// $ ./bin --flag



fn main() {
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