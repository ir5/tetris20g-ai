use wasm_bindgen::prelude::*;
use tetris20g_ai::core::{ScoreInfo, Field, EMPTY_FIELD};
use tetris20g_ai::agent::TwoStepSearchAgent;
use tetris20g_ai::enumeration::find_command_sequence;
use tetris20g_ai::utility;
use tetris20g_ai::regressor::LinearRegressor;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct GameManager {
    agent: TwoStepSearchAgent,
    field: Field,
    seq: Vec<u8>,
    score_info: ScoreInfo,
}

#[wasm_bindgen]
impl GameManager {
    pub fn new(param_string: &str, seq_string: &str) -> GameManager {
        let agent = TwoStepSearchAgent::new_direct(&param_string);
        let field = EMPTY_FIELD;
        let seq: Vec<u8> = seq_string.bytes().collect();
        let score_info = ScoreInfo::new();

        GameManager {
            agent,
            field,
            seq,
            score_info
        }
    }

    pub fn step(&mut self) {
    }
}

#[wasm_bindgen]
pub fn greet(param_string: &str) {
    let agent = TwoStepSearchAgent::new_direct(param_string);
    let mut reg = LinearRegressor::new();
    reg.load_direct("1 2 3");
}
