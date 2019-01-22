use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;
use std::sync::{mpsc, Arc, Mutex};
use csv::{ReaderBuilder, StringRecord};

// use retrosheet::{Parser, ParserError};

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

    //let (dbtx, dbrx) = mpsc::channel();

    // for i in 1..5 {
    //     parser(i, dbtx.clone());
    // }

    // let _ = db_storer(Arc::new(Mutex::new(dbrx))).join();

    let (parser_tx, parser_rx) = mpsc::channel();
    for file in files {
        parser(file, parser_tx.clone());
    }
    drop(parser_tx);
    
    let _ = db_storer(Arc::new(Mutex::new(parser_rx))).join();
    // send the last of the games over
    //println!("I should send this over to the processor: {:?}", raw_game);
}

fn parser(file_path: String, dbtx: mpsc::Sender<Vec<StringRecord>>) {
    thread::spawn(move || {
        let file = File::open(file_path).unwrap();
        let mut buf_reader = BufReader::new(file);
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_reader(buf_reader);

    let mut raw_game: Vec<StringRecord> = Vec::new();
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result.unwrap();
        let record_type = &record[0];
        //println!("record_type: {:?}", record_type);
        if record_type == "id" {
            if raw_game.len() > 0 {
                println!("I shouuld send the game, for now I'm just clearing the raw_game vec");
                let send_err = dbtx.send(raw_game.clone());
                    
                if let Err(e) = send_err {
                    eprintln!("ERROR: {:?}", e);
                }

                raw_game.clear()
            }  else {
                println!("I have a new file!")

            }
        }

        raw_game.push(record.clone());
        //println!("{:?}", &record[0]);
    }
    drop(dbtx);
    println!("at the end of the loop");
    });
}

fn db_storer(rx: Arc<Mutex<mpsc::Receiver<Vec<StringRecord>>>>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        for v in rx.lock().unwrap().iter() {
            println!("db_storer got: {:?}", v);
        }
    })
}