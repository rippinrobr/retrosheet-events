use std::default::Default;
use csv::{StringRecord};
use regex::Regex;

#[derive(Clone, Debug, Default)]
pub struct Starter {
    pub player_id: String,
    pub name: String,
    pub team: u8, // this is 0 for visiting team, 1 for home team
    pub batting_order: u8,
    pub position: u8, // dh is 10 everyone else is the normal numeric position value
}

impl Starter {
    pub fn new(record: StringRecord) -> Self {
        let re = Regex::new(r"[A-Za-z]").unwrap();
        let raw_pos = record[5].to_string();
        let clean_pos = re.replace_all(raw_pos.trim(), "");
        let position: u8 = match clean_pos.parse::<u8>() {
            Ok(n) => n,
            Err(e) => {
                eprintln!("[EarnedRunEntry::new] an error occurred while attempting to parse '{}' into a u8", record[3].to_string());
                20
            }
        };
        return Self {
            player_id: record[1].to_string(),
            name: record[2].to_string(),
            team: record[3].to_string().parse::<u8>().unwrap(),
            batting_order: record[4].to_string().parse::<u8>().unwrap(),
            position,
        }
    }
}
