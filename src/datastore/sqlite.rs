use std::collections::HashMap;
use sqlite;
use sqlite::{Connection, Value};
use crate::datastore::{DBError, Repository};
use crate::datastore::DBError::{GeneralError, InsertError};
use crate::game::Game;
use crate::game::starter::Starter;
use crate::game::com::Com;
use crate::game::earned_run_entry::EarnedRunEntry;
use crate::game::play::Play;
use crate::game::sub::Sub;
use super::{cleanse_name, swap_unknown_for_numeric_cols};

/// Manages interactions with a Postgres database
pub struct SQLite{
    conn: Connection,
    duplicate_err_msg: String,
}

impl SQLite {
    /// returns an instance of the SQLite which is is used to interact with a SQLite
    /// database
    pub fn new(conn: Connection) -> Self {
        Self {
            conn,
            duplicate_err_msg: String::from("UNIQUE constraint failed"),
        }
    }

    fn insert_coms(&self, game_id: String, coms: Vec<Com>) -> Result<u64, DBError> {
        let total = coms.len() as u64;
        for com in coms {
            let insert_stmt = &format!("INSERT INTO coms (game_id, idx, description) VALUES ('{}', {}, '{}')",
                                       &game_id, com.idx, com.description.replace("'", "''"));
            if let Err(e) = self.conn.execute(insert_stmt) {
                if !format!("{}",e).contains(&self.duplicate_err_msg) {
                    return Err(InsertError {
                        db_type: String::from("[SQLITE]"),
                        message: format!("insert_coms: {}", e)
                    });
                }
                println!("duplicate key message: {}", e);

            }
        }

        Ok(total)
    }

    fn insert_data(&self, game_id: String, data: Vec<EarnedRunEntry>) -> Result<u64, DBError> {
        let total = data.len() as u64;
        for d in data {
            let insert_stmt = &format!("INSERT INTO data (game_id, player_id, er) VALUES ('{}', '{}', {})",
                                       &game_id, d.player_id, d.earned_runs_allowed);
            if let Err(e) = self.conn.execute(insert_stmt) {
                if !format!("{}",e).contains(&self.duplicate_err_msg) {
                    return Err(InsertError {
                        db_type: String::from("[SQLITE]"),
                        message : format!("insert_data: {}", e)});
                }
                println!("duplicate key message: insert_data: {}", e);

            }
        }

        Ok(total)
    }

    /// insert_game_info adds data to the `games` table, the data is stored on the Game struct in
    /// the info Hashmap
    ///
    /// returns the number of records added or a DBError::InsertError
    fn insert_game_info(&self, game_id: String, season: i32, info: HashMap<String, String>)  -> Result<u64, DBError> {
        let insert_stmt = &format!("INSERT INTO games (game_id, season, visteam, hometeam, game_date, number, starttime, daynight, usedh, pitches, umphome, ump1b, \
         ump2b, ump3b, umplf, umprf, fieldcond, precip, sky, temp, winddir, windspeed, timeofgame, attendance, \
         site, wp, lp, save, gwrbi, edittime, howscored, inputprogvers, inputter, inputtime, scorer, translator) VALUES ( \
         '{}', {}, '{}', '{}', '{}', {}, '{}', \
         '{}', '{}', '{}', '{}', '{}', '{}', \
         '{}', '{}', '{}', '{}', '{}', '{}',\
          {}, '{}', {}, {}, {}, \
          '{}', '{}', '{}', '{}', '{}', '{}', \
          '{}', '{}', '{}', '{}', '{}', '{}');",
          &game_id, season, &info["visteam"],&info["hometeam"], &info["date"], &info["number"], &info["starttime"],
          &info["daynight"], &info["usedh"], &info["pitches"], cleanse_name(info["umphome"].clone()),
           cleanse_name(info["ump1b"].clone()),
           cleanse_name(info["ump2b"].clone()), cleanse_name(info["ump3b"].clone()), cleanse_name(info["umplf"].clone()),
           cleanse_name(info["umprf"].clone()), &info["fieldcond"], &info["precip"], &info["sky"],
           swap_unknown_for_numeric_cols(info["temp"].to_string()), &info["winddir"],
           swap_unknown_for_numeric_cols(info["windspeed"].to_string()), &info["timeofgame"], &info["attendance"],
          &info["site"], &info["wp"], &info["lp"], &info["save"], &info["gwrbi"], &info["edittime"],
          &info["howscored"], &info["inputprogvers"], cleanse_name(info["inputter"].clone()), &info["inputtime"],
           cleanse_name(info["scorer"].clone()), cleanse_name(info["translator"].clone()));

        match self.conn.execute(insert_stmt) {
            Ok(rows_added) => Ok(1),
            Err(e) => {
                return Err(InsertError {
                        db_type: String::from("[SQLITE]"),
                        message : format!("insert_game_info: ({}) {}", &game_id, e)});
            }
        }
    }

    fn insert_plays(&self, game_id: String, plays: Vec<Play>) -> Result<u64, DBError> {
        let total = plays.len() as u64;
        for p in plays {
            let insert_stmt = &format!("INSERT INTO plays (game_id, idx, inning, team, player_id, count, pitches, event) \
                VALUES ('{}', {}, {}, '{}', '{}', '{}', '{}', '{}')", &game_id, p.idx, p.inning, p.team,
                p.player_id, p.count, p.pitches, p.event);
            if let Err(e) = self.conn.execute(insert_stmt) {
                if !format!("{}",e).contains(&self.duplicate_err_msg) {
                    return Err(InsertError {
                        db_type: String::from("[SQLITE]"),
                        message : format!("insert_plays: ({}) {}", &game_id, e)});
                }
                println!("duplicate key message: insert_plays: {}", e);

            }
        }

        Ok(total)
    }

    fn insert_starters(&self, game_id: String, starters: Vec<Starter>) -> Result<u64, DBError> {
        let total_starters = starters.len();
        for starter in starters {
            let insert_stmt = &format!("INSERT INTO starters (game_id, player_id, name, team, \
                   batting_order, position) VALUES ('{}', '{}', '{}', '{}', {}, {})", &game_id,
                   starter.player_id, cleanse_name(starter.name), starter.team, starter.batting_order, starter.position);

            if let Err(e) = self.conn.execute(insert_stmt) {
                if !format!("{}",e).contains(&self.duplicate_err_msg) {
                    return Err(InsertError {
                        db_type: String::from("[SQLITE]"),
                        message : format!("insert_starters: ({}) {}",&game_id, e)});
                }
                println!("duplicate key message: insert_starters: {}", e);

            }
        }

        Ok(total_starters as u64)
    }

    fn insert_subs(&self, game_id: String, subs: Vec<Sub>) -> Result<u64, DBError> {
        let total = subs.len() as u64;
        for s in subs {
            let insert_stmt = &format!("INSERT INTO subs (game_id, idx, player_id, name, \
               team, batting_order, position) VALUES ('{}', {}, '{}', '{}', '{}', {}, {})",
               &game_id, s.idx, s.player_id, cleanse_name(s.name), s.team, s.batting_order, s.position);

            if let Err(e) = self.conn.execute(insert_stmt) {
                if !format!("{}",e).contains(&self.duplicate_err_msg) {
                    return Err(InsertError {
                        db_type: String::from("[SQLITE]"),
                        message : format!("insert_subs: ({}) {}", &game_id, e)});
                }
                println!("duplicate key message: insert_subs: {}", e);
            }
        }

        Ok(total)
    }

    pub fn get_duplicate_err_msg(&self) -> String {
        return self.duplicate_err_msg.clone()
    }
}

impl Repository for SQLite {
    fn save_game(&self, game: Game) -> Result<(), DBError> {
        // create the first entry for this game in the database, bails if there's an
        // error
        match self.insert_game_info(game.id.clone(), game.season, game.info) {
            Err(e) => {
                return Err(e)
            }
            _ => ()
        };

        match self.insert_starters(game.id.clone(), game.starters) {
            Err(e) => {
                return Err(e)
            }
            _ => ()
        }

        match self.insert_coms(game.id.clone(), game.coms) {
            Err(e) => return Err(e),
            _ => ()
        }

        match self.insert_data(game.id.clone(), game.data) {
            Err(e) => return Err(e),
            _ => ()
        }

        match self.insert_plays(game.id.clone(), game.plays) {
            Err(e) => return Err(e),
            _ => ()
        }


        match self.insert_subs(game.id.clone(), game.subs) {
            Err(e) => return Err(e),
            _ => Ok(())
        }
    }
}