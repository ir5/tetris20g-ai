extern crate pancurses;

use core::Game;

pub struct Display {
    window: pancurses::Window,
}

impl Display {
    pub fn new() -> Display {
        let window = pancurses::initscr();
        window.keypad(true);
        pancurses::cbreak();
        pancurses::noecho();
        pancurses::start_color();
        pancurses::curs_set(0);
        pancurses::init_color(b'I' as i16, 900, 0, 0);
        pancurses::init_color(b'O' as i16, 900, 900, 0);
        pancurses::init_color(b'S' as i16, 1000, 100, 1000);
        pancurses::init_color(b'Z' as i16, 100, 1000, 100);
        pancurses::init_color(b'L' as i16, 1000, 600, 0);
        pancurses::init_color(b'J' as i16, 200, 200, 1000);
        pancurses::init_color(b'T' as i16, 0, 800, 800);
        pancurses::init_color(1i16, 200, 200, 200);
        pancurses::init_color(2i16, 0, 0, 0);
        pancurses::init_color(3i16, 800, 800, 800);

        for c in "IOSZLJT".bytes() {
            pancurses::init_pair(c as i16, 3, c as i16);
        }
        pancurses::init_pair(b'.' as i16, 2, 1);

        Display { window }
    }

    pub fn draw(&self, game: &Game) {
        self.window.erase();
        for (i, &row) in game.field.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                self.window.mv(i as i32, j as i32);
                self.window.attrset(pancurses::COLOR_PAIR(cell as u64));
                self.window.addch(if cell == b'.' { '.' } else { ' ' });
            }
        }
        self.window.refresh();
    }

    pub fn wait_key(&self) -> Option<char> {
        match self.window.getch() {
            Some(pancurses::Input::Character(c)) => Some(c),
            _ => None,
        }
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        pancurses::endwin();
    }
}
