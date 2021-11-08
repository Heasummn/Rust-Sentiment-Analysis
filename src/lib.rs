mod message; 
pub use message::Message;

use sentiment::*;

pub mod analysis { 
    pub fn analyze_sentiment(m: String) -> sentiment::Analysis {
        return sentiment::analyze(m);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
