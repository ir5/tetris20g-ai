extern crate rand;

extern crate tetris20g_ai;

use rand::Rng;

use tetris20g_ai::core;
use tetris20g_ai::display::Display;
use tetris20g_ai::enumeration::enumerate_multi;

fn main() {
    let mut field = core::EMPTY_FIELD;
    let mut rng = rand::thread_rng();
    for i in 0..9 {
        for j in 0..core::WIDTH {
            field[core::HEIGHT - 1 - i][j] = if rng.gen_range(0, 2) == 0 { b'.' } else { b'X' };
        }
    }
    let candidates = enumerate_multi(&field, &vec![b'L', b'S']);
    println!("{}", candidates.len());
    let display = Display::new();
    let mut idx = 0;

    loop {
        let field = &candidates[idx][1].new_field;
        let state = core::new_piece(b'O');
        display.draw(&field, &state, None);
        let key = display.wait_key();
        if let Some(_) = key {
            idx += 1;
            if idx >= candidates.len() {
                break;
            }
        }
    }
}