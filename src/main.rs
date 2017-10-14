extern crate rand;
extern crate pancurses;

mod core;
mod display;

use core::Game;
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
    let mut game = Game::new(seq);
    let display = Display::new();
    loop {
        display.draw(&game.field, &game.state, game.next_piece());
        let key = display.wait_key();
        if let Some(key) = key {
            game.input(key);
        }
    }
}
