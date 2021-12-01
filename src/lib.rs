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

    pub fn read_from_file(filename: &str) -> Vec<sentiment::Analysis>{

        let file = File::open(filename).expect("Error reading file");
        let buf = BufReader::new(file);
        let inputs:Vec<String> = buf.lines() .map(|l| l.expect("Could not parse line")).collect();

        return strings_to_analyses(inputs);    
    }

    pub fn strings_to_analyses(inputs: Vec<String>) -> Vec<sentiment::Analysis>{
        let mut to_return:Vec<sentiment::Analysis> = Vec::new();

        for s in inputs{
            let a = analyze_sentiment(s);
            to_return.push(a);
        }

        return to_return;
    }

}

pub mod map_reduce{
    use std::collections::HashMap;
    use std::thread;

    use crate::analysis;
    use crate::message::Message; 
    use chrono::{DateTime, Utc};

    use std::sync::Mutex;
    use std::sync::Arc;


    pub fn map_reduce( input: Vec<Message>, num_chunks: usize) 
            -> HashMap<DateTime<Utc>, sentiment::Analysis> {

        // <-----------STEP 1: MAP (split up input and create a thread for each sub-section that calls analysis)----------->

        // variables to help split input;
        let remainder = input.len() % num_chunks;
        let starting_size = (input.len() - remainder) / num_chunks;

        let mut threads = Vec::new();

        let input_a = Arc::new(Mutex::new(input));

        for i in 0..num_chunks{
            let input_a = Arc::clone(&input_a);

            let handle = thread::spawn( move || -> HashMap<DateTime<Utc>, sentiment::Analysis>{  
                let mut chunk_analysis:HashMap<DateTime<Utc>, sentiment::Analysis> = HashMap::new();

                for j in 0..starting_size{
                    let input_m = input_a.lock().unwrap();
                    let m = &input_m[i*starting_size + j];

                    let a = analysis::analyze_sentiment(m.text.to_owned());
                    chunk_analysis.insert(m.time,a);
                }

                return chunk_analysis;
            });
            threads.push(handle);
        }
        // handling remainder;
        let input_a = Arc::clone(&input_a);
        let handle = thread::spawn( move || -> HashMap<DateTime<Utc>, sentiment::Analysis>{  
            let mut chunk_analysis:HashMap<DateTime<Utc>, sentiment::Analysis> = HashMap::new();

            for j in 0..remainder{
                let input_m = input_a.lock().unwrap();
                let m = &input_m[num_chunks*starting_size + j];

                let a = analysis::analyze_sentiment(m.text.to_owned());
                chunk_analysis.insert(m.time,a);
            }

            return chunk_analysis;
        });
        threads.push(handle);
        
        // <-------------STEP 2: REDUCE (aggregate into one result map)------------->

        let mut result = HashMap::new();  

        for handle in threads{
            let map:HashMap<DateTime<Utc>, sentiment::Analysis> = handle.join().unwrap();
            for (time, a) in map{
                result.insert(time.to_owned(), a);  
            }
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
