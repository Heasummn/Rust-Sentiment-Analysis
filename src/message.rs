use chrono::{DateTime, Utc};

pub struct Message {
    text: String,
    time: DateTime<Utc>
}

impl Message {
    pub fn new(text: String, time: DateTime<Utc>) -> Message {
        Message {
            text,
            time
        }
    } 
}