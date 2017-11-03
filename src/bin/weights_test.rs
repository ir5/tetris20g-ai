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
#[structopt(name = "weights_test", about = "Check if learned weights are appropriate.")]
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
    for i in 0..9 {
        for j in 0..core::WIDTH {
            field[core::HEIGHT - 1 - i][j] = if rng.gen_range(0, 2) == 0 { b'.' } else { b'X' };
        }
    }
    let candidates = enumerate_multi(&field, &vec![b'L', b'S']);
    let display = Display::new();

    let mut sorted: Vec<(f32, core::Field)> = candidates
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
