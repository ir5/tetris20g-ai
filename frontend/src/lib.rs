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
    i_command: usize,
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
        let i_command = 0;
        let state = core::new_piece(b'I'); // dummy piece

        GameManager {
            agent,
            field,
            seq,
            score_info,
            step,
            commands,
            i_command,
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
                let y = (i as i32) + (self.state.y as i32);
                if y < 0 {
                    continue;
                }
                let y = y as usize;
                let x = ((j as i32) + (self.state.x as i32)) as usize;
                current[y * core::WIDTH + x] = self.state.piece_type;
            }
        }

        current
    }

    pub fn render_next_piece(&self) -> Vec<u8> {
        let mut disp: Vec<u8> = vec![b'.'; 4 * 4];
        let next_piece_type = self.seq[(self.step + 1) % self.seq.len()];

        if next_piece_type == b'I' {
            // I is special...
            disp[3 * 4 + 0] = b'I';
            disp[3 * 4 + 1] = b'I';
            disp[3 * 4 + 2] = b'I';
            disp[3 * 4 + 3] = b'I';
        } else {
            let shape = core::shape(next_piece_type, 0);
            for (i, &row) in shape.iter().enumerate() {
                for (j, cell) in row.bytes().enumerate() {
                    if cell == b'#' {
                        disp[4 * i + j] = next_piece_type;
                    }
                }
            }
        }

        disp
    }

    pub fn del_counts(&self) -> Vec<usize> {
        self.score_info.del_counts.to_vec()
    }

    pub fn total_lines(&self) -> usize {
        self.score_info.total_lines
    }

    pub fn steps(&self) -> usize {
        self.score_info.steps
    }

    fn reset(&mut self) {
        self.field = core::EMPTY_FIELD;
        self.step = (self.step + 10) % self.seq.len();
        self.score_info = core::ScoreInfo::new();
        self.state = core::new_piece(b'I'); // dummy piece
        self.commands = vec![];
        self.i_command = 0;
    }

    fn next_prediction(&mut self) {
        let next_piece = self.seq[self.step % self.seq.len()];
        let next2_piece = self.seq[(self.step + 1) % self.seq.len()];

        let prediction = self.agent.predict(&self.field, next_piece, next2_piece);

        let dest_state = match prediction {
            None => { self.reset(); return; },
            Some(state) => state,
        };

        self.commands = enumeration::find_command_sequence(&self.field, next_piece, &dest_state);
        self.state = core::new_piece(next_piece);
        self.i_command = 0;
    }

    pub fn act(&mut self) {
        if self.i_command >= self.commands.len() {
            self.next_prediction();
        }

        let command = &self.commands[self.i_command];
        match core::apply_command(&self.field, &self.state, &command) {
            core::CommandResult::Moved(new_state, _) => {
                self.state = new_state;
            }
            core::CommandResult::Fixed(info) => {
                self.field = info.new_field.clone();
                self.score_info.update(info.del);
                self.step += 1;
            }
            _ => (),
        }
        self.i_command += 1;

        if self.i_command >= self.commands.len() {
            self.next_prediction();
        }
    }
}

#[wasm_bindgen]
pub fn greet(param_string: &str) {
    let agent = TwoStepSearchAgent::new_direct(param_string);
    let mut reg = LinearRegressor::new();
    reg.load_direct("1 2 3");
}
