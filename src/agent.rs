use std::f32;

use core::{Field, PieceState};
use enumeration::enumerate_multi;
use regressor::LinearRegressor;

pub trait Agent {
    fn predict(&mut self, field: &Field, next_piece: u8, next2_piece: u8) -> Option<PieceState>;
    fn report(&self) -> String;
}

pub struct TwoStepSearchAgent {
    regressor: LinearRegressor,
    report_string: String,
}

impl TwoStepSearchAgent {
    pub fn new(weights_file: &str) -> TwoStepSearchAgent {
        let mut regressor = LinearRegressor::new();
        regressor.load(&weights_file);
        let report_string = String::from("");
        TwoStepSearchAgent { regressor, report_string }
    }
}

impl Agent for TwoStepSearchAgent {
    fn predict(&mut self, field: &Field, next_piece: u8, next2_piece: u8) -> Option<PieceState> {
        let candidates = enumerate_multi(&field, &vec![next_piece, next2_piece]);
        // find maximum value candidate
        let mut best_value: f32 = f32::MIN;
        let mut best = vec![];
        for candidate in candidates {
            let value = self.regressor.predict(&candidate[1].new_field);
            if value > best_value {
                best_value = value;
                best = candidate.clone();
            }
        }
        if best.is_empty() {
            return None;
        }
        let state = best[0].last_state.clone();

        self.report_string = format!("Value: {}", best_value);

        Some(state)
    }

    fn report(&self) -> String {
        self.report_string.clone()
    }
}