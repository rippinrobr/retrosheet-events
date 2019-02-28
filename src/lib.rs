
pub mod datastore;
pub mod game;

// struct Config {
//     mysql_database_url: String,
//     pg_database_url: String,
//     sqlite_database_url: String,

//     asg_event_dir: String,
//     post_season_event_dir: String,
//     regular_season_event_dir: String
// }

// impl Config {
//     pub fn is_valid(&self) -> Result<(), Vec<String>> {
//         let mut errors: Vec<String> = Vec::new();

//         if self.mysql_database_url.is_empty() {
//             errors.push(String::from("MySql connection string is empty. (MYSQL_DATABASE_URL)"));
//         }

//         if self.pg_database_url.is_empty() {
//             errors.push(String::from("Postgres connection string is empty (PG_DATABASE_URL)."));
//         }

//         if self.sqlite_database_url.is_empty() {
//             errors.push(String::from("SQLite connection string is empty (SQLITE_DATABASE_URL)."));
//         }

//         if self.asg_event_dir.is_empty() {
//             errors.push(String::from("All Star game event directory path was empty (ALLSTAR_GAME_EVENTS). "))
//         }

//         if self.post_season_event_dir.is_empty() {
//             errors.push(String::from("All Star game event directory path was empty (POST_SEASON_EVENTS). "))
//         }

//         if self.regular_season_event_dir.is_empty() {
//             errors.push(String::from("All Star game event directory path was empty (REGULAR_SEASON_EVENTS). "))
//         }

//         if errors.len() > 0 {
//             return Err(errors.clone());
//         }

//         Ok(())
//     }
// }