use sentiment_analyzer::analysis;

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