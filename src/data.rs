use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Note {
    datetime: DateTime<Local>,
    note: String,
}

impl Note {
    pub fn create(note: String) -> Note {
        Note {
            datetime: Local::now(),
            note,
        }
    }

    pub fn contains(&self, keyword: &str) -> bool {
        self.note.contains(keyword)
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}\n\nCreated at: {}",
            self.note,
            self.datetime.format("%v %r")
        )
    }
}
