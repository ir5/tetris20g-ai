extern crate pancurses;

mod display;
mod core;

use display::Display;
use core::{Field, CurrentPieceState, Game};

fn main() {
    let mut f = core::EMPTY_FIELD;
    f[19][0] = b'I';

    let game = Game {
        field: f,
        state: CurrentPieceState { piece_type: b'I', x: 0, y: 0, rotation: 0 },
        piece_array: vec![],
        current_piece_id: 0,
    };

    let display = Display::new();
    display.draw(&game);
    let res = display.wait_key();
}
