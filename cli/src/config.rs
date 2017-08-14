extern crate argparse;
use argparse::{ArgumentParser, StoreTrue, Store};
use conway::world::World_Options;
use conway::world::builder::InitialState;
use conway::world::rules::input_cells::InputCells;
use std::env;
use std::error::Error;
//use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    //* array of cellstate display chars
    pub alive_char: char,
    pub dead_char: char,
    pub output_file: String,
    pub initial_state: String,
    pub adjacent_rules: String,
    pub ruleset: String,
    pub delay: u8,
    pub width: u64,
    pub height: u64,
}
impl Config {
    fn parse_initial_state(x: String) -> InitialState {
        match Some(&*x.to_string()) {
            Some("Random") => InitialState::Random,
        }
    }
    fn parse_adjacent_rules(x: String) -> InputCells {
    }
    pub fn return_options(&self) -> World_Options {
        World_Options {
            width_height: (self.width, self.height),
            init: Config::parse_initial_state(self.initial_state),
            input_cells: Config::parse_adjacent_rules(self.adjacent_rules),
        }
    }
}
