use chrono::{DateTime, Utc};

pub struct Message {
    pub text: String,
    pub time: DateTime<Utc>
}

impl Message {
    pub fn new(text: String, time: DateTime<Utc>) -> Message {
        Message {
            text,
            time
        }
    } 
}