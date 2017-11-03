extern crate rand;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

extern crate tetris20g_ai;

use std::io::{Write, stdout};
use structopt::StructOpt;

use tetris20g_ai::agent;
use tetris20g_ai::agent::Agent;
use tetris20g_ai::core;
use tetris20g_ai::display::Display;
use tetris20g_ai::utility;

#[derive(StructOpt, Debug)]
#[structopt(name = "closed_loop", about = "Closed loop execution of learned policy.")]
struct Opt {
    #[structopt(long = "file", help = "Weights file name.")]
    file: String,

    #[structopt(long = "auto", help = "Automatic execution flag.")]
    auto: bool,

    #[structopt(long = "episodes", default_value = "1",
    help = "The number of episodes to calculate performance statistics.")]
    episodes: u32,
}

fn main() {
    let opt = Opt::from_args();

    let mut agent = agent::TwoStepSearchAgent::new(&opt.file);

    let mut field = core::EMPTY_FIELD;
    let seq = utility::generate_pieces(100000, None);
    let display = if opt.auto { None } else { Some(Display::new()) };

    for episode in 1..(1 + opt.episodes) {
        for i in 0.. {
            let next_piece = seq[i % seq.len()];
            let next2_piece = seq[(i + 1) % seq.len()];
            let prediction = agent.predict(&field, next_piece, next2_piece);

            let state = match prediction {
                None => break,
                Some(state) => state,
            };

            print!("\rEpisode: {}, Step: {}, {}", episode, i, agent.report());
            stdout().flush().unwrap();
            if let Some(ref display) = display {
                display.draw(&field, &state, Some(next2_piece));
                let _ = display.wait_key();
            }

            let (new_field, _) = core::fix_piece(&field, &state);
            field = new_field.clone();
        }
    }
}
