pub mod postgres;
pub mod sqlite;

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


fn cleanse_name(name: String) -> String {
    name.replace("'", "''").replace("\"", "")
}

#[derive(Clone, Debug)]
pub struct DBConfig {
    mysql_conn_url: String,
    pg_conn_url: String,
    sqlite_conn_url: String,
}

impl DBConfig {
    pub fn new(mysql_conn_url: String, pg_conn_url: String, sqlite_conn_url:String) -> Self {
        Self {
            mysql_conn_url,
            pg_conn_url,
            sqlite_conn_url,
        }
    }

    pub fn get_mysql_url(self) -> String {
        return self.mysql_conn_url
    }

    pub fn get_pg_url(self) -> String {
        return self.pg_conn_url
    }

    pub fn get_sqlite_url(self) -> String {
        return self.sqlite_conn_url
    }
}