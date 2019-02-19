use std::collections::HashMap;
use postgres::{Connection};
use postgres::transaction::Transaction;
use crate::game::Game;
use crate::game::starter::Starter;
use crate::datastore::{DBError, Repository};
use crate::datastore::DBError::{GeneralError, InsertError};

/// Manages interactions with a Postgres database
pub struct Postgres{
    conn: Connection,
}

impl Postgres {
    /// returns an instance of the PostgresStore which is is used to interact with a Postgres
    /// database server
    pub fn new(conn: Connection) -> Self {
        Self { conn }
    }

    fn insert_game_info(&self, transaction: &Transaction, game_id: String, info: HashMap<String, String>)  -> Result<u64, DBError> {
        let insert_stmt = &format!("INSERT INTO games (game_id, visteam, hometeam, game_date, number, starttime, daynight, usedh, pitches, umphome, ump1b, \
         ump2b, ump3b, umplf, umprf, fieldcond, precip, sky, temp, winddir, windspeed, timeofgame, attendance, \
         site, wp, lp, save, gwrbi, edittime, howscored, inputprogvers, inputter, inputtime, scorer, translator) VALUES ( \
         '{}', '{}', '{}', '{}', {}, '{}', \
         '{}', {}, '{}', '{}', '{}', '{}', \
         '{}', '{}', '{}', '{}', '{}', '{}',\
          {}, '{}', {}, {}, {}, \
          '{}', '{}', '{}', '{}', '{}', '{}', \
          '{}', '{}', '{}', '{}', '{}', '{}');",
          &game_id, &info["visteam"],&info["hometeam"], &info["date"], &info["number"], &info["starttime"],
          &info["daynight"], &info["usedh"], &info["pitches"], &info["umphome"], &info["ump1b"], &info["ump2b"],
          &info["ump3b"], &info["umplf"], &info["umprf"], &info["fieldcond"], &info["precip"], &info["sky"],
          &info["temp"], &info["winddir"], &info["windspeed"], &info["timeofgame"], &info["attendance"],
          &info["site"], &info["wp"], &info["lp"], &info["save"], &info["gwrbi"], &info["edittime"],
          &info["howscored"], &info["inputprogvers"], &info["inputter"], &info["inputtime"], &info["scorer"], &info["translator"]);

        match transaction.execute(insert_stmt, &[]) {
            Ok(rows_added) => Ok(rows_added),
            Err(e) => {
                eprintln!("postgres: insert statement: {}", insert_stmt);
                Err(InsertError{message: format!("postgress: {}", e)})
            }
        }
    }

    fn insert_starters(&self, transaction: &Transaction, game_id: String, starters: Vec<Starter>) -> Result<u64, DBError> {
        Err(GeneralError {message: String::from("not implemented")})
    }
}

impl Repository for Postgres {
    fn save_game(&self, game: Game) -> Result<(), DBError> {
        let trans = self.conn.transaction().unwrap();

        // create the first entry for this game in the database, bails if there's an
        // error
        match self.insert_game_info(&trans, game.id, game.info) {
            Err(e) => {
                trans.set_rollback();
                return Err(e)
            },
            _ => {
                // trans.commit();
                ()
            }
        };

        Err(DBError::GeneralError{message: "not implemented".to_string()})
        // Add
    }
}