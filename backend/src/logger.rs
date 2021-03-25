//! Module for saving and loading annotated data.
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use core::{Field, PieceState};
use serde_json;

/// A single log data.
#[derive(Serialize, Deserialize, Debug)]
pub struct LogInfo {
    pub field: Field,
    pub decided: PieceState,
    pub next_piece: u8,
    pub step: i32,
}

/// A struct for saving log files.
pub struct Logger {
    file: File,
}

impl Logger {
    pub fn new(filename: &str) -> Logger {
        let file = OpenOptions::new()
            .append(true)
            .create_new(true)
            .open(filename)
            .unwrap();
        Logger { file }
    }

    pub fn save(&mut self, log_info: &LogInfo) {
        let serialized = serde_json::to_string(&log_info).unwrap();
        self.file
            .write(format!("{}\n", serialized).as_bytes())
            .unwrap();
    }
}

/// Loads log information.
pub fn load_log_file(filename: &str) -> Vec<LogInfo> {
    let mut file = OpenOptions::new().read(true).open(filename).unwrap();
    let mut all = String::new();
    file.read_to_string(&mut all).unwrap();
    let mut ret = vec![];
    for line in all.lines() {
        let log_info = serde_json::from_str(&line).unwrap();
        ret.push(log_info);
    }
    ret
}
