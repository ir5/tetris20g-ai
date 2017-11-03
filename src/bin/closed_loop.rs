extern crate rand;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

extern crate tetris20g_ai;

use rand::Rng;
use structopt::StructOpt;

use tetris20g_ai::core;
use tetris20g_ai::display::Display;
use tetris20g_ai::enumeration::enumerate_multi;
use tetris20g_ai::regressor::LinearRegressor;

#[derive(StructOpt, Debug)]
#[structopt(name = "closed_loop", about = "Closed loop execution of learned policy.")]
struct Opt {
    #[structopt(long = "file", help = "Weights file name.")]
    file: String,
}

fn main() {
    let opt = Opt::from_args();

    let mut regressor = LinearRegressor::new();
    regressor.load(&opt.file);

    let mut field = core::EMPTY_FIELD;
    let mut rng = rand::thread_rng();
    let m: Vec<u8> = "IOSZJLT".bytes().collect();
    let mut seq = vec![];
    for _ in 0..10000 {
        seq.push(*rng.choose(&m).unwrap());
    }
    let display = Display::new();

    for i in 0..(seq.len() - 1) {
        let next_piece = seq[i];
        let next2_piece = seq[i + 1];
        let candidates = enumerate_multi(&field, &vec![next_piece, next2_piece]);
        // find maximum value candidate
        let mut best_value: f32 = -1e10;
        let mut best = vec![];
        for candidate in candidates {
            let value = regressor.predict(&candidate[1].new_field);
            if value > best_value {
                best_value = value;
                best = candidate.clone();
            }
        }
        let state = &best[0].last_state;

        println!("Step {} : Value = {}", i, best_value);

        display.draw(&field, &state, Some(next2_piece));
        let _ = display.wait_key();

        let (new_field, _) = core::fix_piece(&field, &state);
        field = new_field.clone();
    }
}
