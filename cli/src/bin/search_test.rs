extern crate rand;

extern crate tetris20g_ai;

use tetris20g_ai::core;
use cli::display::Display;
use tetris20g_ai::enumeration::enumerate_multi;
use tetris20g_ai::utility;

fn main() {
    let field = utility::filled_field(9, None);
    let mut candidates = enumerate_multi(&field, &vec![b'L', b'S']);
    candidates.sort();
    println!("{}", candidates.len());
    let display = Display::new();
    let mut idx = 0;

    loop {
        let field = &candidates[idx][1].new_field;
        let state = core::new_piece(b'O');
        display.erase();
        display.draw_field(&field, &state, None);
        display.refresh();
        let key = display.wait_key();
        if let Some(_) = key {
            idx += 1;
            if idx >= candidates.len() {
                break;
            }
        }
    }
}
