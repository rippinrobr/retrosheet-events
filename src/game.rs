
use std::default::Default;
use csv::{StringRecord};

#[derive(Clone, Debug,Default)]
pub struct Game {
    pub id: String,
}

impl Game {
    pub fn new(rawgame: Vec<StringRecord>) -> Self {
        Self {
            id: rawgame[0][1].to_string(),
        }
    }
}
