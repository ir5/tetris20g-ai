#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

extern crate rand;
extern crate pancurses;

mod core;
mod display;
mod human_manipulation;
mod logger;

use human_manipulation::Game;
use display::Display;
use rand::Rng;

fn main() {
    // test loop
    let mut rng = rand::thread_rng();
    let m: Vec<u8> = "IOSZJLT".bytes().collect();
    let mut seq = vec![];
    for _ in 0..10000 {
        seq.push(*rng.choose(&m).unwrap());
    }
    let mut game = Game::new(seq, Some("test.txt"));
    let display = Display::new();
    loop {
        display.draw(&game.field, &game.state, game.next_piece());
        let key = display.wait_key();
        if let Some(key) = key {
            game.input(key);
        }
    }
}
