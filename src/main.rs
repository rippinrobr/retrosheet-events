use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;
use std::sync::{mpsc, Arc, Mutex};
use csv::{ReaderBuilder, StringRecord};

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

    // an array of file paths to parse, currently only accepting a single path
    // soon you'll be able to process more than one file, perhaps comma delimited
    // paths, globs, or by directory. I'm not sure yet
    // let files = vec![args[1].clone()];
    let files = args[1].split(",");
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
    let _ = store_game(Arc::new(Mutex::new(parser_rx))).join();
}

fn parser(file_path: String, dbtx: mpsc::Sender<Game>) {
    thread::spawn(move || {
        let file = File::open(file_path).unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .flexible(true)
            .from_reader(buf_reader);

        let mut game: Game = Game::default();
        let mut game_log_idx: u16 = 0;

        for result in rdr.records() {
            let record = result.unwrap();
            let record_type = &record[0];
            match record_type {
                "id"  => {
                    if game.id != ""  {
                        let send_err = dbtx.send(game.clone());
                        if let Err(e) = send_err {
                            eprintln!("ERROR: {:?}", e);
                        }
                        game_log_idx = 0;
                        game = Game::default();
                    }
                    game.id = record[1].to_string();
                },
                "info" => game.add_info(record[1].to_string(), record[2].to_string() ),
                "start" => game.add_starter(record),
                "play" => {
                    game.add_play(record, game_log_idx);
                    game_log_idx = game_log_idx +1;
                },
                "sub" => {
                    game.add_sub(record, game_log_idx);
                    game_log_idx = game_log_idx +1;
                },
                "com" => {
                    game.add_com(record, game_log_idx);
                    game_log_idx = game_log_idx + 1;
                },
                "data" => game.add_earned_run_entry(record),
                _ =>(),
            };
        }

        let send_err = dbtx.send(game.clone());
        if let Err(e) = send_err {
            eprintln!("ERROR: {:?}", e);
        }
        drop(dbtx);
        println!("at the end of the loop");
    });
}

fn store_game(rx: Arc<Mutex<mpsc::Receiver<Game>>>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        for v in rx.lock().unwrap().iter() {
            println!("store_game got: {:#?}\n", v);
        }
    })
}