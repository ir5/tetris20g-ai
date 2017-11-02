use std::fs::OpenOptions;
use std::io::Read;
use core::{HEIGHT, WIDTH, Field, EMPTY_FIELD};

pub fn extract_feature(field: &Field) -> Vec<bool> {
    let mut res = vec![];
    for ai in 0..HEIGHT {
        for aj in 0..WIDTH {
            for di in 0..4 {
                for dj in -3i8..4 {
                    let bi = ai + di;
                    let bj = (aj as i8) + dj;
                    if bi >= HEIGHT || bj < 0 || bj >= WIDTH as i8 {
                        continue;
                    }
                    let bj = bj as usize;
                    if ai == bi && aj == bj {
                        continue;
                    }
                    res.push(field[ai][aj] == b'.' && field[bi][bj] != b'.');
                    res.push(field[ai][aj] != b'.' && field[bi][bj] == b'.');
                }
            }
        }
    }
    res
}

pub struct LinearRegressor {
    params: Vec<f32>,
}

impl LinearRegressor {
    pub fn new() -> LinearRegressor {
        let dim = extract_feature(&EMPTY_FIELD);
        LinearRegressor { params: vec![0.0; dim.len()] }
    }

    pub fn load(&mut self, filename: &str) {
        //! Load space-separated weight file.
        let mut file = OpenOptions::new().read(true).open(filename).unwrap();
        let mut all = String::new();
        file.read_to_string(&mut all).unwrap();
        self.params = all.split_whitespace()
            .map(|x| x.parse::<f32>().unwrap())
            .collect();
    }

    pub fn predict(&self, field: &Field) -> f32 {
        let feature = extract_feature(&field);
        self.params.iter().zip(feature.iter()).fold(
            0.0,
            |sum, (&f, &b)| {
                if b { sum + f } else { sum }
            },
        )
    }
}
