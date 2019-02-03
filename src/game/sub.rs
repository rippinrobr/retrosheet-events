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
        return Self {
            idx,
            player_id: record[1].to_string(),
            name: record[2].to_string(),
            team: record[3].to_string().parse::<u8>().unwrap(),
            batting_order: record[4].to_string().parse::<u8>().unwrap(),
            position: record[5].to_string().parse::<u8>().unwrap(),
        }
    }
}
