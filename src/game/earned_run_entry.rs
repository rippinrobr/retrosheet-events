use std::default::Default;
use csv::{StringRecord};
use regex::Regex;

#[derive(Clone, Debug, Default)]
pub struct EarnedRunEntry {
    pub player_id: String,
    pub earned_runs_allowed: u8,
}

impl EarnedRunEntry {
    pub fn new(record: StringRecord) -> Self {
        let re = Regex::new(r"[A-Za-z#]").unwrap();
        let raw_string = &record[3].to_string();
        let er_string = re.replace_all(raw_string.trim(), "");

        let earned_runs: u8 = match er_string.parse::<u8>() {
            Ok(n) => n,
            Err(e) => {
                eprintln!("[EarnedRunEntry::new] an error occurred while attempting to parse '{}' into a u8", record[3].to_string());
                0
            }
        };

        Self {
            player_id: record[2].to_string(),
            earned_runs_allowed: earned_runs,
        }
    }
}

impl PartialEq for EarnedRunEntry {
    fn eq(&self, other: &EarnedRunEntry) -> bool {
        return &self.player_id == &other.player_id &&
                &self.earned_runs_allowed == &other.earned_runs_allowed;
    }
}