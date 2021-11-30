pub mod message;

pub mod analysis { 
    use crate::message::Message;
    pub fn analyze_sentiment(m: Message) -> sentiment::Analysis {
        return sentiment::analyze(m.text);
    }

    pub fn display(a: &sentiment::Analysis) {
        println!("Overall score: {}", a.score);
        println!("Comparative: {}", a.comparative);
        println!("NEGATIVE:");
        let n = &a.negative;
        println!("{}", n.score);
        println!("{:?}", n.words);
        println!("POSITIVE:");
        let p = &a.positive;
        println!("{}", p.score);
        println!("{:?}", p.words);
        println!("--------------")
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
