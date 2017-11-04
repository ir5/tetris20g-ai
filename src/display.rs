//! Module for displaying a current field, state, and other information
//! in CUI interface.
extern crate pancurses;

use core;
use core::{Field, PieceState};

pub struct Display {
    window: pancurses::Window,
}

impl Display {
    pub fn new() -> Display {
        let window = pancurses::initscr();
        window.keypad(true);

        #[cfg(windows)]
        pancurses::resize_term(30, 40);

        pancurses::cbreak();
        pancurses::noecho();
        pancurses::start_color();
        pancurses::curs_set(0);
        pancurses::init_color(b'I' as i16, 900, 400, 400);
        pancurses::init_color(b'O' as i16, 900, 900, 400);
        pancurses::init_color(b'S' as i16, 1000, 400, 1000);
        pancurses::init_color(b'Z' as i16, 500, 1000, 200);
        pancurses::init_color(b'L' as i16, 1000, 700, 300);
        pancurses::init_color(b'J' as i16, 500, 500, 1000);
        pancurses::init_color(b'T' as i16, 300, 900, 900);
        pancurses::init_color(b'X' as i16, 900, 900, 900);
        pancurses::init_color(1i16, 200, 200, 200);
        pancurses::init_color(2i16, 0, 0, 0);
        pancurses::init_color(3i16, 800, 800, 800);

        for c in "IOSZLJTX".bytes() {
            pancurses::init_pair(c as i16, 3, c as i16);
        }
        pancurses::init_pair(b'.' as i16, 2, 1);

        Display { window }
    }

    pub fn draw(&self, field: &Field, state: &PieceState, next_piece_type: Option<u8>) {
        self.window.erase();
        let offset = 5;
        // draw field
        for (i, &row) in field.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                self.window.mv(offset + i as i32, j as i32);
                self.window.attrset(pancurses::COLOR_PAIR(cell as u64));
                self.window.addch(if cell == b'.' { '.' } else { ' ' });
            }
        }
        // draw current block
        let shape = core::shape(state.piece_type, state.rotation);
        for (i, &row) in shape.iter().enumerate() {
            for (j, cell) in row.bytes().enumerate() {
                if cell == b'.' {
                    continue;
                }
                let y = (i as i32) + (state.y as i32);
                let x = (j as i32) + (state.x as i32);
                self.window.mv(offset + y, x);
                self.window.attrset(
                    pancurses::COLOR_PAIR(state.piece_type as u64),
                );
                self.window.addch('#');
            }
        }
        // draw next block
        if let Some(next_piece_type) = next_piece_type {
            let shape = core::shape(next_piece_type, 0);
            for (i, &row) in shape.iter().enumerate() {
                for (j, cell) in row.bytes().enumerate() {
                    if cell == b'.' {
                        continue;
                    }
                    let y = i as i32;
                    let x = j as i32;
                    self.window.mv(y, 3 + x);
                    self.window.attrset(
                        pancurses::COLOR_PAIR(next_piece_type as u64),
                    );
                    self.window.addch(' ');
                }
            }
        }
        // refresh the window
        self.window.refresh();
    }

    pub fn wait_key(&self) -> Option<char> {
        match self.window.getch() {
            Some(pancurses::Input::Character(c)) => Some(c),
            _ => None,
        }
    }

    pub fn napms(&self, ms: i32) {
        pancurses::napms(ms);
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        pancurses::endwin();
    }
}
