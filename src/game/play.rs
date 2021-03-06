
use std::default::Default;
use std::io::{self, Write};
use csv::{StringRecord};

#[derive(Clone, Debug, Default)]
pub struct Play {
    pub idx: u16, // the index in the play/sub/com log that this play occurred at
    pub inning: u8,
    pub team: u8, // this is 0 for visiting team, 1 for home team
    pub player_id: String,
    pub count: String, // most games do not have this data so they will have ?? in this place
    pub pitches: String, // most games do not have this data so they will be blank
    pub event: String,
}

impl Play {
    pub fn new(idx: u16, record: StringRecord) -> Self {
        let team_num = match record[2].to_string().parse::<u8>() {
            Ok(num) => num,
            Err(e) => {
                let mut n: u8 = 9;
                // this is here because of this line 1963KC1.EVA:play,5,b,gardb101,??,,NP
                // the b is not a 0 or 1 so the parser blew up
                if record[2].to_string() == "b" {
                    n = 0;
                }
                eprintln!("the value at record[2] '{:?}' did not convert to a u8! Using {} instead.", record[2].to_string(), n);
                n
            }
        };

        Self {
            idx,
            inning: record[1].to_string().parse::<u8>().unwrap(),
            team: team_num,
            player_id: record[3].to_string(),
            count: record[4].to_string(),
            pitches: record[5].to_string(),
            event: record[6].to_string(),
        }
    }
}