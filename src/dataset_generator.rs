use std::fs::OpenOptions;
use std::io::{self, Write};
use enumeration::enumerate_multi;
use logger::load_log_file;
use regressor::extract_feature;
use core::EMPTY_FIELD;
use core::fix_piece;
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

pub fn generate_dataset(input: &str, output: &str, drop_rate: f64) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(output)
        .unwrap();

    let log_infos = load_log_file(input);
    let uniform = Range::new(0f64, 1f64);
    let mut rng = rand::thread_rng();

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
        let candidates = enumerate_multi(&field, &vec![current_piece, next_piece]);

        for candidate in candidates {
            if uniform.ind_sample(&mut rng) < drop_rate {
                continue;
            }
            if log_infos[idx].decided == candidate[0].last_state {
                continue;
            }

            let feature0 = extract_feature(&best);
            let feature0 = vecbool_to_vecu8(&feature0);
            let feature1 = extract_feature(&candidate[1].new_field);
            let feature1 = vecbool_to_vecu8(&feature1);

            file.write(&feature0).unwrap();
            file.write(&feature1).unwrap();
        }
    }
}
