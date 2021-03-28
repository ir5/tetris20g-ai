use wasm_bindgen::prelude::*;
use tetris20g_ai::core;
use tetris20g_ai::agent::Agent;
use tetris20g_ai::agent::TwoStepSearchAgent;
use tetris20g_ai::enumeration;
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
    field: core::Field,
    seq: Vec<u8>,
    score_info: core::ScoreInfo,
    step: usize,
    commands: Vec<core::Command>,
    state: core::PieceState,
}

#[wasm_bindgen]
impl GameManager {
    pub fn new(param_string: &str, seq_string: &str) -> GameManager {
        let agent = TwoStepSearchAgent::new_direct(&param_string);
        let field = core::EMPTY_FIELD;
        let seq: Vec<u8> = seq_string.bytes().collect();
        let score_info = core::ScoreInfo::new();
        let step = 0;
        let commands: Vec<core::Command> = vec![];
        let state = core::new_piece(b'I'); // dummy piece

        GameManager {
            agent,
            field,
            seq,
            score_info,
            step,
            commands,
            state,
        }
    }

    pub fn render_field(&self) -> Vec<u8> {
        // returns: (current field, flag of current piece)
        let mut flattened: Vec<u8> = vec![];
        for &row in self.field.iter() {
            flattened.extend(row.to_vec());
        }
        flattened
    }

    pub fn render_current_piece(&self) -> Vec<u8> {
        let mut current: Vec<u8> = vec![b'.'; core::WIDTH * core::HEIGHT];
        let shape = core::shape(self.state.piece_type, self.state.rotation);
        for (i, &row) in shape.iter().enumerate() {
            for (j, cell) in row.bytes().enumerate() {
                if cell == b'.' {
                    continue;
                }
                let y = ((i as i32) + (self.state.y as i32)) as usize;
                let x = ((j as i32) + (self.state.x as i32)) as usize;
                current[y * core::WIDTH + x] = self.state.piece_type;
            }
        }

        current
    }

    pub fn act(&mut self) {
        let next_piece = self.seq[self.step % self.seq.len()];
        let next2_piece = self.seq[(self.step + 1) % self.seq.len()];

        if self.commands.len() == 0 {
            let prediction = self.agent.predict(&self.field, next_piece, next2_piece);

            let dest_state = match prediction {
                None => return,
                Some(state) => state,
            };

            self.commands = enumeration::find_command_sequence(&self.field, next_piece, &dest_state);
            self.state = core::new_piece(next_piece);
        }

        let command = &self.commands[0];
        match core::apply_command(&self.field, &self.state, &command) {
            core::CommandResult::Moved(new_state, _) => {
                self.state = new_state;
            }
            core::CommandResult::Fixed(info) => {
                self.field = info.new_field.clone();
                self.score_info.update(info.del);
            }
            _ => (),
        }
    }
}

#[wasm_bindgen]
pub fn greet(param_string: &str) {
    let agent = TwoStepSearchAgent::new_direct(param_string);
    let mut reg = LinearRegressor::new();
    reg.load_direct("1 2 3");
}
