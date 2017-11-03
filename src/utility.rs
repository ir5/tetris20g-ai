//! Bunch of utility functions.
use rand::{thread_rng, Rng, SeedableRng, XorShiftRng};

use core;

pub fn generate_pieces(len: usize, seed: Option<u32>) -> Vec<u8> {
    let seed = match seed {
        None => thread_rng().gen::<u32>(),
        Some(seed) => seed,
    };
    let mut rng: XorShiftRng = SeedableRng::from_seed([seed; 4]);
    let m: Vec<u8> = "IOSZJLT".bytes().collect();
    let mut seq = vec![];
    for _ in 0..len {
        seq.push(*rng.choose(&m).unwrap());
    }
    seq
}

pub fn filled_field(lines: usize, seed: Option<u32>) -> core::Field {
    let seed = match seed {
        None => thread_rng().gen::<u32>(),
        Some(seed) => seed,
    };
    let mut rng: XorShiftRng = SeedableRng::from_seed([seed; 4]);

    let mut field = core::EMPTY_FIELD;
    for i in 0..lines {
        for j in 0..core::WIDTH {
            field[core::HEIGHT - 1 - i][j] = if rng.gen_range(0, 2) == 0 { b'.' } else { b'X' };
        }
    }
    field
}
