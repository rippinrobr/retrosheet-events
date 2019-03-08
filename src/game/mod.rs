
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
    pub season: i32,
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
        let new_data = EarnedRunEntry::new(record);
        for d in &self.data {
            if d.eq(&new_data) {
                println!("[{}-{}] data: {} {} already exists in vector", &self.season, &self.id, d.player_id, d.earned_runs_allowed);
                return
            }
        }
        self.data.push(new_data)
    }

    pub fn add_info(&mut self, prop: String, val: String) {
        self.info.insert(prop, val);
    }

    pub fn add_play(&mut self, record: StringRecord, idx: u16) { self.plays.push( Play::new(idx, record)); }

    pub fn add_starter(&mut self, record: StringRecord) {
        self.starters.push(Starter::new(record))
    }

    pub fn add_sub(&mut self, record: StringRecord, idx: u16) { self.subs.push( Sub::new(record, idx)); }
    pub fn set_default_info(&mut self) {
        &self.info.insert(String::from("visteam"), String::new());
        &self.info.insert(String::from("hometeam"), String::new());
        &self.info.insert(String::from("game_date"), String::new());
        &self.info.insert(String::from("number"), String::new());
        &self.info.insert(String::from("starttime"), String::new());
        &self.info.insert(String::from("daynight"), String::new());
        &self.info.insert(String::from("usedh"), String::new());
        &self.info.insert(String::from("pitches"), String::new());
        &self.info.insert(String::from("umphome"), String::new());
        &self.info.insert(String::from("ump1b"), String::new());
        &self.info.insert(String::from("ump2b"), String::new());
        &self.info.insert(String::from("ump3b"), String::new());
        &self.info.insert(String::from("umplf"), String::new());
        &self.info.insert(String::from("umprf"), String::new());
        &self.info.insert(String::from("fieldcond"), String::new());
        &self.info.insert(String::from("precip"), String::new());
        &self.info.insert(String::from("sky"), String::new());
        &self.info.insert(String::from("temp"), String::new());
        &self.info.insert(String::from("winddir"), String::new());
        &self.info.insert(String::from("windspeed"), String::from("0"));
        &self.info.insert(String::from("timeofgame"), String::new());
        &self.info.insert(String::from("attendance"), String::new());
        &self.info.insert(String::from("site"), String::new());
        &self.info.insert(String::from("wp"), String::new());
        &self.info.insert(String::from("lp"), String::new());
        &self.info.insert(String::from("save"), String::new());
        &self.info.insert(String::from("gwrbi"), String::new());
        &self.info.insert(String::from("edittime"), String::new());
        &self.info.insert(String::from("howscored"), String::new());
        &self.info.insert(String::from("inputprogvers"), String::new());
        &self.info.insert(String::from("inputter"), String::new());
        &self.info.insert(String::from("inputtime"), String::new());
        &self.info.insert(String::from("scorer"), String::new());
        &self.info.insert(String::from("translator"), String::new());
    }

}
