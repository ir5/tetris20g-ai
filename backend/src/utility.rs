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

/// Returns the average and the standard deviation of given values.
pub fn statistics(scores: &Vec<f64>) -> (f64, f64) {
    let n = scores.len() as f64;
    let average = scores.iter().sum::<f64>() / n;
    let stdev = if scores.len() == 1 { 0.0 } else {
        (scores.iter().map(|x| (x - average).powi(2)).sum::<f64>() / (n - 1.0)).sqrt()
    };

    (average, stdev)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_statistics() {
        let scores = vec![1.0, 2.0, 3.0, 10.0];
        let (average, stdev) = statistics(&scores);
        assert!((average - 4.0).abs() < 1e-9);
        assert!((stdev - (50.0f64 / 3.0).sqrt()).abs() < 1e-9);
    }
}