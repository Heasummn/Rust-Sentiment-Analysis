use sentiment_analyzer::analysis;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    // let messages = vec!()
    let happy_words = "absolutely accepted acclaimed accomplish accomplishment achievement action active admire adorable adventure affirmative affluent agree agreeable amazing angelic appealing approve aptitude attractive awesome";
    let sad_words = "blue dejected depressed despondent down droopy hangdog inconsolable low melancholic melancholy mirthless sad unhappy woebegone woeful";
    
    // let temp = analysis::analyze_sentiment("good good!".to_string());
    // display(temp);
    
    let temp2 = analysis::analyze_sentiment("good good good good!".to_string());
    display(temp2);
    
    // let good_temp = analysis::analyze_sentiment(happy_words.to_string());
    // let bad_temp = analysis::analyze_sentiment(sad_words.to_string());

    // display(good_temp);
    // display(bad_temp);

    //---- testing read_from_file ----
    // let test = read_from_file("data/test-input-simple.txt");
    let test = read_from_file("data/test-input-pride.txt");

    for a in test{
        display(a);
    }
}

fn display(a: sentiment::Analysis) {
    println!("Overall score: {}", a.score);
    println!("Comparative: {}", a.comparative);
    println!("NEGATIVE:");
    let n = a.negative;
    println!("{}", n.score);
    println!("{:?}", n.words);
    println!("POSITIVE:");
    let p = a.positive;
    println!("{}", p.score);
    println!("{:?}", p.words);
    println!("--------------")
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