extern crate pancurses;
extern crate rand;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

extern crate tetris20g_ai;

use std::time;
use std::thread::sleep;
use structopt::StructOpt;

use tetris20g_ai::agent;
use tetris20g_ai::agent::Agent;
use tetris20g_ai::core;
use tetris20g_ai::display::Display;
use tetris20g_ai::enumeration::find_command_sequence;
use tetris20g_ai::utility;

#[derive(StructOpt, Debug)]
#[structopt(name = "demo", about = "Demonstration version.")]
struct Opt {
    #[structopt(long = "file", default_value = "weights__1.txt",
    help = "Weights file name.")]
    file: String,

    #[structopt(long = "lines", default_value = "8",
    help = "The number of lines initially filled at random.")]
    lines: usize,
}

fn main() {
    let opt = Opt::from_args();

    let display = Display::new();

    loop {
        let mut agent = agent::TwoStepSearchAgent::new(&opt.file);

        let mut field = utility::filled_field(opt.lines, None);
        let seq = utility::generate_pieces(100000, None);

        for step in 0.. {
            let next_piece = seq[step % seq.len()];
            let next2_piece = seq[(step + 1) % seq.len()];
            let prediction = agent.predict(&field, next_piece, next2_piece);

            let dest_state = match prediction {
                None => break,
                Some(state) => state,
            };

            let seq = find_command_sequence(&field, next_piece, &dest_state);
            let mut state = core::new_piece(next_piece);
            for command in seq {
                display.draw(&field, &state, Some(next2_piece));
                let _ = display.wait_key();
                // pancurses::half_delay(2);
                // sleep(time::Duration::from_millis(10000));

                match core::apply_command(&field, &state, &command) {
                    core::CommandResult::Moved(new_state, _) => {
                        state = new_state;
                    }
                    core::CommandResult::Fixed(info) => {
                        field = info.new_field.clone();
                    }
                    _ => (),
                }
            }
        }
    }
}
