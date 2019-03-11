use std::default::Default;
use csv::{StringRecord};

#[derive(Clone, Debug, Default)]
pub struct Sub {
    pub idx: u16, // the index in the play/sub/com log that this play occurred at
    pub player_id: String,
    pub name: String,
    pub team: u8, // this is 0 for visiting team, 1 for home team
    pub batting_order: u8,
    pub position: u8, // dh is 10, 11 is pinch hitter and 12 is pinch runner
}

impl Sub {
    pub fn new(record: StringRecord, idx: u16) -> Self {
        let team: u8 = match  record[3].to_string().trim().parse::<u8>() {
            Ok(n) => n,
            Err(e) => {
                eprintln!("[Sub::new] unable to parse team number '{}' int u8, defaulting to 9", record[3].to_string());
                9
            }
        };

        let batting_order: u8 = match  record[4].to_string().trim().parse::<u8>() {
            Ok(n) => n,
            Err(e) => {
                eprintln!("[Sub::new] unable to parse batting slot '{}' int u8, defaulting to 15", record[4].to_string());
                15
            }
        };

        let position: u8 = match  record[5].to_string().trim().parse::<u8>() {
            Ok(n) => n,
            Err(e) => {
                eprintln!("[Sub::new] unable to parse position '{}' int u8, defaulting to 15", record[5].to_string());
                15
            }
        };

        return Self {
            idx,
            player_id: record[1].to_string(),
            name: record[2].to_string(),
            team,
            batting_order,
            position,
        }
    }
}
