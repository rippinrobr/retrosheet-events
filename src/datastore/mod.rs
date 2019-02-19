pub mod postgres;


// use std::error::Error;
// use std::result::Result;
use failure::{Backtrace, Fail};
//use postgres::{Connection};
//use postgres::transaction::Transaction;
use crate::game::Game;
//use crate::game::starter::Starter;
use crate::datastore::DBError::{GeneralError, InsertError};


#[derive(Debug, Fail)]
 pub enum DBError {
     #[fail(display = "db error: {}", message)]
     GeneralError { message: String },

     #[fail(display = "db error: {}", message)]
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

