mod message; 
pub use message::Message;

// use sentiment::*; // why not needed?

pub mod analysis { 
    use std::collections::HashMap;
        
    use std::fs::File;
    use std::io::BufReader;
    use std::io::BufRead;

    use crate::message::Message; // TODO: why are these needed?
    use chrono::{DateTime, Utc};

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

    /*
    Assumptions I made;
        - input will be read from a txt file. if it's a different format, this should be pretty easy to adjust
    */
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

    pub fn messages_to_time_analyses_map(inputs: &Vec<&Message>) -> HashMap<DateTime<Utc>, Box<sentiment::Analysis>>{
        let mut to_return:HashMap<DateTime<Utc>, Box<sentiment::Analysis>> = HashMap::new();

        for m in inputs{
            let a = analyze_sentiment(m.text.to_owned());
            to_return.insert(m.time,Box::new(a));
        }

        return to_return;
    }
}


// TODO: maybe move to another file?
pub mod map_reduce{
    use std::sync::{mpsc, mpsc::Receiver};
    use std::collections::HashMap;
    use std::thread::JoinHandle;
    // use std::hash::Hash; // why not needed?
    use std::thread;

    use crate::analysis;
    use crate::message::Message; // TODO: why are these needed?
    use chrono::{DateTime, Utc};
    
    pub fn split_data_into_chunks(items: &Vec<Message>, num_chunks: usize) 
            -> Vec<Vec<&Message>> {
        let mut result = Vec::new();

        let remainder = items.len() % num_chunks;
        let starting_size = (items.len() - remainder) / num_chunks;

        let mut counter = 0;
        for _i in 0..num_chunks{
            let mut sub_vector = Vec::new();
            for _j in 0 .. starting_size{
                sub_vector.push(&items[counter]);
                counter+= 1;
            }
            result.push(sub_vector);
        }

        // assign remainder to sub-vectors
        for j in 0..remainder{
            result[j].push(&items[counter]);
            counter+= 1;
        }

        return result;
    }

    pub fn multi_threaded_mapper(input: &Vec<Message>, num_chunks: usize) 
            -> Vec<(JoinHandle<()>, Receiver<HashMap<DateTime<Utc>, Box<sentiment::Analysis>>>)> {
        let mut result = Vec::new();

        let split_input = split_data_into_chunks(input, num_chunks);

        for i in 0..num_chunks{
            let split_input_chunk = &split_input[i];
            //spawn a new thread
            let (tx, rx) = mpsc::channel();
            let handle = thread::spawn(move ||{   // might need to use channels? or 'crossbeam scope'?
                //call analysis
                let chunk_analysis = analysis::messages_to_time_analyses_map(split_input_chunk);  

                tx.send(chunk_analysis).unwrap();
            });
            //create tuple of joinhandle and receiver
            let tuple = (handle, rx);
            //add tuple to result
            result.push(tuple);
        }

        return result;
    }

    pub fn thread_reducer( receivers: Vec<(JoinHandle<()>, Receiver<HashMap<DateTime<Utc>, Box<sentiment::Analysis>>>)>) 
        -> HashMap<DateTime<Utc>, Box<sentiment::Analysis>> {
    let mut result = HashMap::new();  

        for (_handle, rx) in receivers{
            let thread_map = rx.recv().unwrap();

            for (time, a) in thread_map.iter(){
                result.insert(time.to_owned(), *a);  // TODO: try heap-allocating the analyses and pass around pointers instead?
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
