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

use chrono::prelude::*;
use human_manipulation::Game;
use display::Display;
use rand::Rng;

fn timestamp() -> String {
    let local: DateTime<Local> = Local::now();
    String::from(local.format("%Y%m%d-%H%M%S").to_string())
}

fn main() {
    let mut opts = getopts::Options::new();
    opts.reqopt("", "mode", "mode name", "MODE NAME");
    opts.optopt("", "lines", "initial random lines", "NUM");

    let args: Vec<String> = std::env::args().collect();
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(e) => { panic!(e.to_string()) }
    };
    let mode = matches.opt_str("mode").unwrap_or(String::from("annotation"));
    let initial_lines = matches.opt_str("lines").unwrap_or(String::from("0")).parse::<usize>().unwrap();

    if mode == "annotation" {
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
                game.field[core::HEIGHT - 1 - i][j] = if rng.gen_range(0, 2) == 0 { b'.' } else { b'X' };
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
}
