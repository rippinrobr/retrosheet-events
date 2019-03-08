use std::default::Default;
use csv::{StringRecord};

#[derive(Clone, Debug, Default)]
pub struct EarnedRunEntry {
    pub player_id: String,
    pub earned_runs_allowed: u8,
}

impl EarnedRunEntry {
    pub fn new(record: StringRecord) -> Self {
        Self {
            player_id: record[2].to_string(),
            earned_runs_allowed: record[3].to_string().parse::<u8>().unwrap(),
        }
    }
}

impl PartialEq for EarnedRunEntry {
    fn eq(&self, other: &EarnedRunEntry) -> bool {
        return &self.player_id == &other.player_id &&
                &self.earned_runs_allowed == &other.earned_runs_allowed;
    }
}