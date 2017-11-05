extern crate rand;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

extern crate tetris20g_ai;

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
    #[structopt(long = "file", default_value = "resources/weights__1.txt",
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
        let mut score_info = core::ScoreInfo::new();

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
                display.erase();
                display.draw_field(&field, &state, Some(next2_piece));
                display.draw_score_info(&score_info);
                display.refresh();
                display.napms(50);

                match core::apply_command(&field, &state, &command) {
                    core::CommandResult::Moved(new_state, _) => {
                        state = new_state;
                    }
                    core::CommandResult::Fixed(info) => {
                        field = info.new_field.clone();
                        score_info.update(info.del);
                    }
                    _ => (),
                }
            }
            display.napms(100);
        }
    }
}
