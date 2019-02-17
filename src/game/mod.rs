
pub mod com;
pub mod earned_run_entry;
pub mod play;
pub mod starter;
pub mod sub;

use std::collections::HashMap;
use std::default::Default;
use csv::{StringRecord};
use crate::game::com::Com;
use crate::game::earned_run_entry::EarnedRunEntry;
use crate::game::play::Play;
use crate::game::starter::Starter;
use crate::game::sub::Sub;

#[derive(Clone, Debug,Default)]
pub struct Game {
    pub id: String,
    pub info: HashMap<String, String>,
    pub starters: Vec<Starter>,
    pub plays: Vec<Play>,
    pub coms: Vec<Com>,
    pub subs: Vec<Sub>,
    pub data: Vec<EarnedRunEntry>,
}

impl Game {
    pub fn add_com(&mut self, record: StringRecord, idx: u16) {
        self.coms.push(Com::new(idx, record))
    }

    pub fn add_earned_run_entry(&mut self, record: StringRecord) {
        self.data.push(EarnedRunEntry::new(record))
    }

    pub fn add_info(&mut self, prop: String, val: String) {
        self.info.insert(prop, val);
    }

    pub fn add_play(&mut self, record: StringRecord, idx: u16) { self.plays.push( Play::new(idx, record)); }

    pub fn add_starter(&mut self, record: StringRecord) {
        self.starters.push(Starter::new(record))
    }

    pub fn add_sub(&mut self, record: StringRecord, idx: u16) { self.subs.push( Sub::new(record, idx)); }
}