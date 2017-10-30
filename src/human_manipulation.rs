use core::{Field, CurrentPieceState, EMPTY_FIELD, Command, new_piece, apply_command, CommandResult};
use logger::{Logger, LogInfo};

pub struct Game {
    pub field: Field,
    pub state: CurrentPieceState,
    pub piece_array: Vec<u8>,
    pub current_piece_id: usize,
    pub charge: i32,
    logger: Option<Logger>,
    prev_log_info: Option<LogInfo>,
    step: i32,
}

impl Game {
    pub fn new(piece_array: Vec<u8>, filename: Option<String>) -> Game {
        let mut logger = None;
        if let Some(filename) = filename {
            logger = Some(Logger::new(&filename));
        }
        Game {
            field: EMPTY_FIELD,
            state: new_piece(piece_array[0]),
            piece_array,
            current_piece_id: 0,
            charge: 0,
            logger: logger,
            prev_log_info: None,
            step: 0,
        }
    }

    pub fn input(&mut self, key: char) {
        let mut dx: i32 = 0;
        let command = match key {
            'z' => {
                dx = -1;
                Some(Command::Move(dx as i8, 0))
            }
            'x' => Some(Command::Fix),
            'c' => {
                dx = 1;
                Some(Command::Move(dx as i8, 0))
            }
            'm' | ',' | '.' => {
                let rotate = if key == ',' { -1 } else { 1 };
                let synchro = if self.charge.abs() < 6 {
                    0
                } else {
                    self.charge.signum() as i8
                };
                Some(Command::Move(synchro, rotate))
            }
            'r' => {
                self.state = new_piece(self.state.piece_type);
                None
            }
            'n' => {
                self.prev_log_info = None;
                None
            }
            _ => None,
        };

        if let Some(command) = command {
            let res = apply_command(&self.field, &self.state, &command);
            match res {
                CommandResult::Moved(next_state, _) => {
                    if self.state == next_state {
                        if self.charge * dx <= 0 {
                            self.charge = dx;
                        } else {
                            self.charge += dx;
                        }
                    } else {
                        self.state = next_state;
                        self.charge = 0;
                    }

                    if let Command::Move(0, _) = command {
                        self.charge = 0;
                    }
                }
                CommandResult::Fixed(info) => {
                    self.state = info.last_state;
                    self.update_log();
                    self.field = info.new_field;
                    self.state = new_piece(self.next_piece().unwrap());
                    self.current_piece_id += 1;
                    self.charge = 0;
                    self.step += 1;
                }
                CommandResult::Ended => {
                    panic!();
                }
            }
        }
    }

    pub fn next_piece(&self) -> Option<u8> {
        if self.current_piece_id + 1 < self.piece_array.len() {
            Some(self.piece_array[self.current_piece_id + 1])
        } else {
            None
        }
    }
    
    pub fn update_log(&mut self) {
        if let Some(ref prev_log_info) = self.prev_log_info {
            if let Some(ref mut logger) = self.logger {
                logger.save(prev_log_info);
            }
        }
        let log_info = LogInfo {
            field: self.field,
            decided: self.state.clone(),
            next_piece: self.next_piece().unwrap(),
            step: self.step,
        };
        self.prev_log_info = Some(log_info);
    }
}
