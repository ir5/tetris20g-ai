#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate rand;
extern crate pancurses;
// extern crate getopts;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;
extern crate chrono;

mod core;
mod display;
mod human_manipulation;
mod logger;
mod enumeration;
mod regressor;
mod dataset_generator;

use core::Field;
use chrono::prelude::*;
use human_manipulation::Game;
use display::Display;
use rand::Rng;
use enumeration::enumerate_multi;
use dataset_generator::generate_dataset;
use structopt::StructOpt;
use regressor::LinearRegressor;


#[derive(StructOpt, Debug)]
#[structopt(name = "20G")]
enum Opt {
    #[structopt(name = "annotation")]
    Annotation {
        #[structopt(long = "lines", default_value = "0")]
        lines: usize,

        #[structopt(long = "save-file")]
        save_file: Option<String>,

        #[structopt(long = "no-save")]
        no_save: bool,
    },

    #[structopt(name = "search-test")]
    SearchTest {},

    #[structopt(name = "generate-dataset")]
    GenerateDataset {
        #[structopt(long = "input")]
        input: String,

        #[structopt(long = "output", default_value = "output.bin")]
        output: String,

        #[structopt(long = "drop-rate", default_value = "0")]
        drop_rate: f64,

        #[structopt(long = "weights-file")]
        weights_file: Option<String>,
    },

    #[structopt(name = "check-weights")]
    CheckWeights {
        #[structopt(long = "file")]
        file: String,
    },
}

fn main() {
    let opt = Opt::from_args();

    match opt {
        Opt::Annotation {
            lines,
            save_file,
            no_save,
        } => {
            annotation(lines, save_file, no_save);
        }
        Opt::SearchTest {} => search_test(),
        Opt::GenerateDataset {
            input,
            output,
            drop_rate,
            weights_file,
        } => {
            generate_dataset(&input, &output, drop_rate, weights_file);
        }
        Opt::CheckWeights { file } => check_weights(file),
    }
}

fn annotation(initial_lines: usize, save_file: Option<String>, no_save: bool) {
    let mut rng = rand::thread_rng();
    let m: Vec<u8> = "IOSZJLT".bytes().collect();
    let mut seq = vec![];
    for _ in 0..10000 {
        seq.push(*rng.choose(&m).unwrap());
    }

    let save_file: Option<String> = if no_save {
        None
    } else {
        if let Some(name) = save_file {
            Some(name)
        } else {
            Some(format!("dataset/{}.txt", timestamp()))
        }
    };

    let mut game = Game::new(seq, save_file);
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

fn check_weights(file: String) {
    let mut regressor = LinearRegressor::new();
    regressor.load(&file);

    let mut field = core::EMPTY_FIELD;
    let mut rng = rand::thread_rng();
    for i in 0..9 {
        for j in 0..core::WIDTH {
            field[core::HEIGHT - 1 - i][j] = if rng.gen_range(0, 2) == 0 { b'.' } else { b'X' };
        }
    }
    let candidates = enumerate_multi(&field, &vec![b'L', b'S']);
    let display = Display::new();

    let mut sorted: Vec<(f32, Field)> = candidates
        .iter()
        .map(|e| (regressor.predict(&e[1].new_field), e[1].new_field))
        .collect();
    sorted.sort_by(|x, y| y.0.partial_cmp(&x.0).unwrap());

    for (value, field) in sorted {
        let state = core::new_piece(b'O');
        display.draw(&field, &state, None);

        println!("Value = {}", value);

        let _ = display.wait_key();
    }
}

fn timestamp() -> String {
    let local: DateTime<Local> = Local::now();
    String::from(local.format("%Y%m%d-%H%M%S").to_string())
}
