//! Implementation for creation of 20G tetris AI.
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate rand;
extern crate pancurses;

pub mod agent;
pub mod core;
pub mod dataset_generator;
pub mod display;
pub mod enumeration;
pub mod human_manipulation;
pub mod logger;
pub mod regressor;
pub mod utility;
