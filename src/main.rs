extern crate pancurses;

mod display;
mod core;

use core::Game;
use display::Display;

fn main() {
    // test loop
    let mut seq = vec![];
    for i in 0..1000 {
        let m: Vec<u8> = "IOSZJLT".bytes().collect();
        let idx = i % 7;
        seq.push(m[idx]);
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
