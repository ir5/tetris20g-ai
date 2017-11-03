extern crate chrono;
extern crate rand;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

extern crate tetris20g_ai;

use chrono::prelude::*;
use rand::Rng;
use structopt::StructOpt;

use tetris20g_ai::core;
use tetris20g_ai::display::Display;
use tetris20g_ai::human_manipulation::Game;

#[derive(StructOpt, Debug)]
#[structopt(name = "annotation", about = "Create annotation data from human play.")]
struct Opt {
    #[structopt(long = "lines", default_value = "0",
                help = "The number of lines initially filled.")]
    lines: usize,

    #[structopt(long = "save-file", help = "File name for saving annotation log data.")]
    save_file: Option<String>,

    #[structopt(long = "no-save", help = "The program will not save log data if true.")]
    no_save: bool,
}

fn main() {
    let opt = Opt::from_args();

    let mut rng = rand::thread_rng();
    let m: Vec<u8> = "IOSZJLT".bytes().collect();
    let mut seq = vec![];
    for _ in 0..10000 {
        seq.push(*rng.choose(&m).unwrap());
    }

    let save_file: Option<String> = if opt.no_save {
        None
    } else {
        if let Some(name) = opt.save_file {
            Some(name)
        } else {
            Some(format!("dataset/{}.txt", timestamp()))
        }
    };

    let mut game = Game::new(seq, save_file);
    for i in 0..opt.lines {
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

fn timestamp() -> String {
    let local: DateTime<Local> = Local::now();
    String::from(local.format("%Y%m%d-%H%M%S").to_string())
}
