use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;
use std::sync::{mpsc, Arc, Mutex};
use csv::{ReaderBuilder, StringRecord};
use dotenv::dotenv;
use postgres::{Connection,TlsMode};
use retrosheet_loader::datastore::Repository;
use retrosheet_loader::datastore::postgres::Postgres;
use retrosheet_loader::game::{
    Game,
    play::Play,
};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    // need to have at least one file
    if args.len() < 2 {
        eprintln!("path to Retrosheet event file is missing");
        std::process::exit(1);
    }

    dotenv().ok();
    let pg_conn_str = env::var("PG_DATABASE_URL")
        .expect("DATABASE_URL must be set");
    // an array of file paths to parse, currently only accepting a single path
    // soon you'll be able to process more than one file, perhaps comma delimited
    // paths, globs, or by directory. I'm not sure yet
    // let files = vec![args[1].clone()];
    let files = args[1].split(',');
    // Sets up the channels that are used for sending parsed Game objects to from
    // the parser to the function responsible for storing the data in a database
    let (parser_tx, parser_rx) = mpsc::channel();
    // loops over the paths provided and sends them to the parser function, each
    // file being parsed will have its own thread.
    for file in files.clone() {
        parser(file.to_string(), parser_tx.clone());
    }
    // when I'm done parsing all of the files I can close the channel that is used by the parser
    // to send games to the store_game thread
    drop(parser_tx);

    // the store_game function is on its own thread listening for Game objects being sent to
    // it from the parser function
    let _ = store_game(Arc::new(Mutex::new(parser_rx)), pg_conn_str.clone()).join();
}

fn parser(file_path: String, dbtx: mpsc::Sender<Game>) {
    thread::spawn(move || {
        let file = File::open(file_path).unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .flexible(true)
            .from_reader(buf_reader);

        let mut retro_game: Game = Game::default();
        retro_game.set_default_info();
        let mut game_log_idx: u16 = 0;

        for result in rdr.records() {
            let record = result.unwrap();
            let record_type = &record[0];
            match record_type {
                "id"  => {
                    if retro_game.id != ""  {
                        let send_err = dbtx.send(retro_game.clone());
                        if let Err(e) = send_err {
                            eprintln!("ERROR: {:?}", e);
                        }
                        game_log_idx = 0;
                        retro_game = Game::default();
                        retro_game.set_default_info();
                    }
                    retro_game.id = record[1].to_string();
                },
                "info" => retro_game.add_info(record[1].to_string(), record[2].to_string() ),
                "start" => retro_game.add_starter(record),
                "play" => {
                    retro_game.add_play(record, game_log_idx);
                    game_log_idx += 1;
                },
                "sub" => {
                    retro_game.add_sub(record, game_log_idx);
                    game_log_idx += 1;
                },
                "com" => {
                    retro_game.add_com(record, game_log_idx);
                    game_log_idx += 1;
                },
                "data" => retro_game.add_earned_run_entry(record),
                _ =>(),
            };
        }

        let send_err = dbtx.send(retro_game.clone());
        if let Err(e) = send_err {
            eprintln!("ERROR: {:?}", e);
        }
        drop(dbtx);
    });
}

fn store_game(rx: Arc<Mutex<mpsc::Receiver<Game>>>, pg_conn_str: String) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let pg_conn = Connection::connect(pg_conn_str, TlsMode::None).unwrap();
        let pg_repo = Postgres::new(pg_conn);

        for g in rx.lock().unwrap().iter() {
            match pg_repo.save_game(g.clone()) {
                    Err(e) => eprintln!("{}",e),
                    Ok(_) => println!("do something here, like update a progress bar"),
                }

        }
    })
}