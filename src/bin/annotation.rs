extern crate chrono;
extern crate rand;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

extern crate tetris20g_ai;

use chrono::prelude::*;
use structopt::StructOpt;

use tetris20g_ai::display::Display;
use tetris20g_ai::human_manipulation::Game;
use tetris20g_ai::utility;

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

    let seq = utility::generate_pieces(100000, None);
    let save_file: Option<String> = save_file_name(&opt);
    let mut game = Game::new(seq, save_file);
    game.field = utility::filled_field(opt.lines, None);

    let display = Display::new();
    loop {
        display.draw(&game.field, &game.state, game.next_piece());
        let key = display.wait_key();
        if let Some(key) = key {
            game.input(key);
        }
    }
}

fn save_file_name(opt: &Opt) -> Option<String> {
    if opt.no_save {
        None
    } else {
        if let Some(ref name) = opt.save_file {
            Some(name.clone())
        } else {
            Some(format!("dataset/{}.txt", timestamp()))
        }
    }
}

fn timestamp() -> String {
    let local: DateTime<Local> = Local::now();
    String::from(local.format("%Y%m%d-%H%M%S").to_string())
}
