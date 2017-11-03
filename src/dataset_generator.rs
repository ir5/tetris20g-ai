//! Module for generating a dataset for optimizing policy parameters.

use std::fs::OpenOptions;
use std::io::{self, Write};
use enumeration::enumerate_multi;
use logger::load_log_file;
use regressor::extract_feature;
use regressor::LinearRegressor;
use core::EMPTY_FIELD;
use core::fix_piece;
use core::Field;
use rand;
use rand::distributions::{IndependentSample, Range};

fn vecbool_to_vecu8(v: &Vec<bool>) -> Vec<u8> {
    let len = v.len();
    let mut res: Vec<u8> = vec![0u8; (len + 7) / 8];
    for i in 0..len {
        if v[i] {
            res[i / 8] ^= 1u8 << (i & 7);
        }
    }
    res
}

/// Output a dataset file for a given log file.
/// Because the number of data can be huge, we can drop some of data specifying `drop_rate`.
/// ### Args:
/// * input: Input log file name. (e.g., `dataset/20171101-000000.txt`)
/// * output: Output binary file name.
/// * drop_rate: Rate for drop. This should be between 0.0 and 1.0.
/// * weights_file: If None, drop is uniformly done for all candidates at random. Otherwise,
/// weights file is loaded and bottom `drop_rate` of candidates in terms of value scores are dropped.
pub fn generate_dataset(input: &str, output: &str, drop_rate: f64, weights_file: Option<String>) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(output)
        .unwrap();

    let log_infos = load_log_file(input);
    let uniform = Range::new(0f64, 1f64);
    let mut rng = rand::thread_rng();

    let mut regressor = LinearRegressor::new();
    if let Some(ref file) = weights_file {
        regressor.load(&file);
    }

    let dim = extract_feature(&EMPTY_FIELD).len();
    println!("dimension = {}", dim);

    for idx in 0..(log_infos.len() - 1) {
        print!("\r{}", idx);
        io::stdout().flush().unwrap();
        if log_infos[idx].step + 1 != log_infos[idx + 1].step {
            // non-continuous frames
            continue;
        }

        let field = log_infos[idx].field;
        let current_piece = log_infos[idx].next_piece;
        let next_piece = log_infos[idx + 1].next_piece;
        let (best, _) = fix_piece(&log_infos[idx + 1].field, &log_infos[idx + 1].decided);
        let mut candidates = enumerate_multi(&field, &vec![current_piece, next_piece]);
        candidates.retain(|e| e[0].last_state != log_infos[idx].decided);

        let candidates: Vec<Field> = if let Some(_) = weights_file {
            // candidates must be sorted by value scores
            let mut sorted: Vec<(f32, Field)> = candidates
                .iter()
                .map(|e| (regressor.predict(&e[1].new_field), e[1].new_field))
                .collect();
            sorted.sort_by(|x, y| y.0.partial_cmp(&x.0).unwrap());
            let mut sorted: Vec<Field> = sorted.iter().map(|e| e.1).collect();

            // the number of elements should be decreased by drop_rate
            let pos = ((1.0 - drop_rate) * (sorted.len() as f64)) as usize;
            sorted.split_off(pos);
            sorted
        } else {
            candidates.retain(|_| uniform.ind_sample(&mut rng) > drop_rate);
            candidates.iter().map(|e| e[1].new_field).collect()
        };

        for candidate in candidates {
            let feature0 = extract_feature(&best);
            let feature0 = vecbool_to_vecu8(&feature0);
            let feature1 = extract_feature(&candidate);
            let feature1 = vecbool_to_vecu8(&feature1);

            file.write(&feature0).unwrap();
            file.write(&feature1).unwrap();
        }
    }
}
