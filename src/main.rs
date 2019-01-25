use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;
use std::sync::{mpsc, Arc, Mutex};
use csv::{ReaderBuilder, StringRecord};

use retrosheet_loader::game::Game;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("path to retrosheet event file is missing");
        std::process::exit(1);
    }

    let file = File::open(&args[1]).unwrap_or_else(|e| {
        eprintln!("ERROR: {}", e);
        
        std::process::exit(2);
    });

    let files = vec![args[1].clone()];
    let (parser_tx, parser_rx) = mpsc::channel();
    for file in files {
        parser(file, parser_tx.clone());
    }
    drop(parser_tx);
    
    let _ = db_storer(Arc::new(Mutex::new(parser_rx))).join();
}

fn parser(file_path: String, dbtx: mpsc::Sender<Game>) {
    thread::spawn(move || {
        let file = File::open(file_path).unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .flexible(true)
            .from_reader(buf_reader);

        let mut raw_game: Vec<StringRecord> = Vec::new();
        let mut game: Game = Game::default();
        let mut game_log_idx: usize = 0;
        for result in rdr.records() {
            // The iterator yields Result<StringRecord, Error>, so we check the
            // error here.
            let record = result.unwrap();
            let record_type = &record[0];
            match record_type {
                "id"  => {
                    if record_type == "id" {
                        if raw_game.len() > 0 {
                            let game = Game::new(raw_game.clone());
                            let send_err = dbtx.send(game.clone());
                            if let Err(e) = send_err {
                                eprintln!("ERROR: {:?}", e);
                            }
                            raw_game.clear();
                            game_log_idx = 0;
                        }
                    }
                },
                "info" => {
                    println!("info: {:?}", record.clone());
                },
                "start" =>{
                    println!("start: {:?}", record.clone());
                },
                "play" => {
                    println!("({}) play: {:?}", game_log_idx, record.clone());
                    game_log_idx = game_log_idx +1;
                },
                "sub" => {
                    println!("({}) sub: {:?}", game_log_idx, record.clone());
                    game_log_idx = game_log_idx +1;
                }
                _ =>(),
            };

            raw_game.push(record.clone());
        }
        drop(dbtx);
        println!("at the end of the loop");
    });
}

fn db_storer(rx: Arc<Mutex<mpsc::Receiver<Game>>>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        for v in rx.lock().unwrap().iter() {
             println!("db_storer got: {:?}", v);
        }
    })
}