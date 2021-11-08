use std::env;

// TO USE, RUN:
// $ rustc src/bin.rs
// $ ./bin --flag

fn main() {
    let arguments: Vec<String> = env::args().collect();

    if arguments.len() < 2 {
        // Temporary error handling - not enough flags passed in
        // add in help message?
        println!("Invalid input! (Case A)")
    }   
    
    let input_format = arguments[1].to_lowercase(); //Second flag in the series (input format)
    let split: Vec<&str> = input_format.split("=").collect(); //Second flag, but split in key-value pair

    if split.len() != 2 {
        // More temporary error handling - invalid flag passed in
        println!("Invalid input! (Case B)");
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