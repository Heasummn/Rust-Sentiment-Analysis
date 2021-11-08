use std::env;
use dialoguer::{Select, theme::ColorfulTheme};

// TO USE, RUN:
// $ rustc src/bin.rs
// $ ./bin --flag



fn main() {
    let arguments: Vec<String> = env::args().collect();

    if arguments.len() < 2 {
        // Using the actual CLI (activates with no command line arguments);
        let selection = initalize_cli();

        match selection {
            0 => println!("Index 0"),
            1 => println!("Index 1"),
            _ => println!("Unseen index!") //Should never happen (new function called for each input format)
        }
        
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

fn initalize_cli() -> usize  {
    let items = vec!["CSV File Input", "Twitter Data"];
    let selection: usize = Select::with_theme(&ColorfulTheme::default())
        .items(&items)
        .default(0)
        .interact()
        .unwrap();    

    return selection;
}