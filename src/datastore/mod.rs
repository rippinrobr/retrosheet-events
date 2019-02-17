use std::collections::HashMap;
// use std::error::Error;
// use std::result::Result;
use failure::{Backtrace, Fail};
use postgres::{Connection};
use postgres::transaction::Transaction;
use crate::game::Game;
use crate::datastore::DBError::{GeneralError, InsertError};

#[derive(Debug, Fail)]
 pub enum DBError {
     #[fail(display = "db error: {}", message)]
     GeneralError { message: String },

     #[fail(display = "DBError::InsertError")]
     InsertError
     {
         message: String,
// //        backtrace: Backtrace,
// //        #[cause]
// //        cause: std::error::Error,
     }
}

pub trait Repository {
    fn save_game(&self, game: Game) -> Result<(), DBError>;
}

// /// Manages interactions with a Postgres database
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
         let insert_stmt = "INSERT INTO games (game_id, visteam, hometeam, game_date, number, starttime, daynight, usedh, pitches, umphome, ump1b, \
         ump2b, ump3b, umplf, umprf, fieldcond, precip, sky, temp, winddir, windspeed, timeofgame, attendance \
         site, wp, lp, save, gwrbi, edittime, howscored, inputprogvers, inputter, inputtime, scorer, translator) VALUES ( \
         $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22 $23, \
         $24, $25, $26, $27, $28, $29, $30, $31, $32, $33, $34 $35, $36";

         match transaction.execute(insert_stmt, &[&game_id, &info["visteam"],&info["hometeam"],&info["game_date"],&info["number"],&info["starttime"],
             &info["daynight"], &info["usedh"], &info["pitches"], &info["umphome"], &info["ump1b"], &info["ump2b"],
             &info["ump3b"], &info["umplf"], &info["umprf"], &info["fieldcond"], &info["precip"], &info["sky"],
             &info["temp"], &info["winddir"], &info["windspeed"], &info["timeofgame"], &info["attendance"],
             &info["site"], &info["wp"], &info["lp"], &info["save"], &info["gwrbi"], &info["edittime"], &info["howscored"],
             &info["inputprogvers"], &info["inputter"], &info["inputtime"], &info["scorer"], &info["translator"]]) {
                 Ok(rows_added) => Ok(rows_added),
                 Err(e) => Err(InsertError{message: format!("{}", e)})
         }
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
             _ => ()
         };

         Err(DBError::GeneralError{message: "not implemented".to_string()})
         // Add
     }
 }