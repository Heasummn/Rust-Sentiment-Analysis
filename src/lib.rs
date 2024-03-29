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

pub mod map_reduce{
    use std::thread;

    use crate::analysis;
    use crate::message::Message; 
    use crate::analysis::AnalysisResult;

    use std::sync::Mutex;
    use std::sync::Arc;

    extern crate num_cpus;

    /*
    Separating away the logic for deciding how many chunks to split the input into.
    It must be less than the input length for mapreduce to not break. 
    Beyond that requirement, apparently for CPU bound loads like we have here the
    ideal number of threads to produce is the number of cores the machine being used has. 
    To find that out, I'm using the num_cpus crate (the get method incls 'hyperthreaded' cores 
    and other tricks like that).
    */
    pub fn pick_num_chunks(input_len: usize) -> usize{
        let num_cores = num_cpus::get();

        if input_len < num_cores{
            return input_len;
        } else {
            return num_cores-1; // the minus one is there bc the result of this function is the number of
                                // additional threads we're creating, and ideally the number of threads this program
                                // creates doesn't surpass the number of usable cores.
        }
    }

    /*
        Analyzes a vector of Messages to return a vector of AnalysisResults, but
        uses multi-threading to speed up the process. Each call of analysis is expensive,
        but this way multiple calls of it can be called simultaneously
    */
    pub fn map_reduce_messages_to_analyses( input: Vec<Message>) 
            -> Vec<AnalysisResult> {
        // STEP 0: deciding number of chunks the input is divided into, which is equal
        // to the number of threads created.
        let num_chunks = pick_num_chunks(input.len());


        // <-----------STEP 1: MAP (split up input and create a thread for each sub-section that calls analysis)----------->

        // variables to help split input;
        // the following code will create one thread for each of num_chunks
        // that analyzes an equal-sized(starting_size) portion of the input.
        // then one last thread will analyze what is left of the input.
        // all the threads will be collected to use in the 'reduce' step later.
        let remainder = input.len() % num_chunks;
        let starting_size = (input.len() - remainder) / num_chunks;

        let mut threads = Vec::new();

        // initial declaration of Arc and Mutex storing input vector
        // - necessary to allow multiple threads to access same vector
        let input_arc = Arc::new(Mutex::new(input));

        for i in 0..num_chunks{
            let input_arc = Arc::clone(&input_arc);
            let handle = thread::spawn( move || -> Vec<AnalysisResult>{  
                let mut chunk_analysis:Vec<AnalysisResult> = Vec::new();

                for j in 0..starting_size{
                    let input_m = input_arc.lock().unwrap();
                    let m = &input_m[i*starting_size + j];

                    // need to manually deep clone Message to pass into analyze_sentiment...
                    let m_deep_clone = Message::new(m.text.to_owned(), m.time);
                    let a = analysis::analyze_sentiment(m_deep_clone); 
                    chunk_analysis.push(a);
                }

                return chunk_analysis;
            });
            threads.push(handle);
        }
        // repeating process to handle remainder inputs;
        let input_arc = Arc::clone(&input_arc);
        let remainder_handle = thread::spawn( move || -> Vec<AnalysisResult>{  
            let mut chunk_analysis:Vec<AnalysisResult> = Vec::new();

            for j in 0..remainder{
                let input_m = input_arc.lock().unwrap();
                let m = &input_m[num_chunks*starting_size + j];

                // need to manually deep clone Message to pass into analyze_sentiment...
                let m_deep_clone = Message::new(m.text.to_owned(), m.time);
                let a = analysis::analyze_sentiment(m_deep_clone); 
                chunk_analysis.push(a);
            }

            return chunk_analysis;
        });
        threads.push(remainder_handle);
        
        // <-------------STEP 2: REDUCE (aggregate into one result map)------------->

        // unwrap each of the threads from step 1 and collect results into 
        // one vector of AnaylsisResults to return.
        let mut result = Vec::new();  

        for handle in threads{
            for a in handle.join().unwrap(){ result.push(a); }
        }

        return result;
    }
}