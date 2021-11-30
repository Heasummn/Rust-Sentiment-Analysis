pub mod message;

pub mod analysis { 
    use crate::message::Message;
    use chrono::{DateTime, Utc};

    pub struct AnalysisResult {
        pub result : sentiment::Analysis,
        pub time : DateTime<Utc>
    }

    impl AnalysisResult {
        pub fn new(result: sentiment::Analysis, time: DateTime<Utc>) -> AnalysisResult {
            AnalysisResult {
                result, time
            }
        }
    }

    pub fn analyze_sentiment(m: Message) -> AnalysisResult {
        return AnalysisResult::new(sentiment::analyze(m.text), m.time);
    }

    pub fn display(r: &AnalysisResult) {
        let a = &r.result;
        println!("Overall score at {}: {}", &r.time.to_rfc2822(), a.score);
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
