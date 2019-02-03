
use std::default::Default;
use csv::{StringRecord};

#[derive(Clone, Debug, Default)]
pub struct Com {
    pub idx: u16, // the index in the play/sub/com log that this play occurred at
    pub description: String, // some times a text description of an event is necessary
}

impl Com {
    pub fn new(idx: u16, record: StringRecord) -> Self {
        Self {
            idx,
            description: record[1].to_string(),
        }
    }
}