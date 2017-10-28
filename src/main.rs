#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate rand;
extern crate pancurses;
extern crate getopts;
extern crate chrono;

mod core;
mod display;
mod human_manipulation;
mod logger;
mod enumeration;
mod regressor;
mod dataset_generator;

use chrono::prelude::*;
use human_manipulation::Game;
use display::Display;
use rand::Rng;
use enumeration::enumerate_multi;
use dataset_generator::generate_dataset;

fn timestamp() -> String {
    let local: DateTime<Local> = Local::now();
    String::from(local.format("%Y%m%d-%H%M%S").to_string())
}

fn main() {
    let mut opts = getopts::Options::new();
    opts.reqopt("", "mode", "mode name", "MODE NAME");
    opts.optopt("", "lines", "initial random lines", "NUM");
    opts.optopt(
        "",
        "filename",
        "filename for generating training dataset",
        "FILENAME",
    );

    let args: Vec<String> = std::env::args().collect();
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => panic!(e.to_string()),
    };
    let mode = matches.opt_str("mode").unwrap();

    match mode.as_str() {
        "annotation" => {
            let initial_lines = matches
                .opt_str("lines")
                .unwrap_or(String::from("0"))
                .parse::<usize>()
                .unwrap();
            annotation(initial_lines);
        }
        "search-test" => search_test(),
        "generate-training-dataset" => {
            let filename = matches.opt_str("filename").unwrap();
            generate_dataset(&filename, "test.bin");
        }
        _ => (),
    };
}

fn annotation(initial_lines: usize) {
    let mut rng = rand::thread_rng();
    let m: Vec<u8> = "IOSZJLT".bytes().collect();
    let mut seq = vec![];
    for _ in 0..10000 {
        seq.push(*rng.choose(&m).unwrap());
    }
    let filename = format!("dataset/{}.txt", timestamp());
    let mut game = Game::new(seq, Some(&filename));
    for i in 0..initial_lines {
        for j in 0..core::WIDTH {
            game.field[core::HEIGHT - 1 - i][j] =
                if rng.gen_range(0, 2) == 0 { b'.' } else { b'X' };
        }
    }

    let display = Display::new();
    loop {
        display.draw(&game.field, &game.state, game.next_piece());
        let key = display.wait_key();
        if let Some(key) = key {
            game.input(key);
        }
    }
}

fn search_test() {
    let mut field = core::EMPTY_FIELD;
    let mut rng = rand::thread_rng();
    for i in 0..9 {
        for j in 0..core::WIDTH {
            field[core::HEIGHT - 1 - i][j] = if rng.gen_range(0, 2) == 0 { b'.' } else { b'X' };
        }
    }
    let candidates = enumerate_multi(&field, &vec![b'L', b'S']);
    println!("{}", candidates.len());
    let display = Display::new();
    let mut idx = 0;

    loop {
        let field = &candidates[idx][1].new_field;
        let state = core::new_piece(b'O');
        display.draw(&field, &state, None);
        let key = display.wait_key();
        if let Some(_) = key {
            idx += 1;
            if idx >= candidates.len() {
                break;
            }
        }
    }
}
