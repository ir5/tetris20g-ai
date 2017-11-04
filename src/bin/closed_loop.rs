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

    let display = if opt.auto { None } else { Some(Display::new()) };
    let mut scores = vec![];

    for episode in 1..(1 + opt.episodes) {
        let mut agent = agent::TwoStepSearchAgent::new(&opt.file);

        let mut field = core::EMPTY_FIELD;
        let seq = utility::generate_pieces(100000, Some(episode));

        for step in 0.. {
            let next_piece = seq[step % seq.len()];
            let next2_piece = seq[(step + 1) % seq.len()];
            let prediction = agent.predict(&field, next_piece, next2_piece);

            let state = match prediction {
                None => {
                    scores.push(step as f64);
                    break
                },
                Some(state) => state,
            };

            print!("\rEpisode: {}, Step: {}, {}", episode, step, agent.report());
            stdout().flush().unwrap();
            if let Some(ref display) = display {
                display.erase();
                display.draw_field(&field, &state, Some(next2_piece));
                display.refresh();
                let _ = display.wait_key();
            }

            let (new_field, _) = core::fix_piece(&field, &state);
            field = new_field.clone();
        }
        println!();
    }

    let (average, stdev) = utility::statistics(&scores);
    println!("Average: {}, Stdev: {}", average, stdev);
}
