mod message; 
pub use message::Message;

use sentiment::*;

pub mod analysis { 
    pub fn analyze_sentiment(m: String) -> sentiment::Analysis {
        return sentiment::analyze(m);
    }

    pub fn display(a: sentiment::Analysis) {
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
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
