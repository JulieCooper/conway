extern crate argparse;
use argparse::{ArgumentParser, StoreTrue, Store};
use conway::world::World_Options;
use conway::world::cell::CellState;
use conway::world::builder::InitialState;
use conway::world::rules::Rulesets;
use conway::world::rules::input_cells::InputCells;
use std::env;
use std::error::Error;
//use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    pub alive_char: char,
    pub dead_char: char,
    pub output_file: String,
    pub initial_state: String,
    pub adjacent_rules: String,
    pub ruleset: String,
    pub delay: u64,
    pub width: u64,
    pub height: u64,
}
impl Config {
    fn parse_initial_state(x: &String) -> InitialState {
        //if let Some(c) = x.chars().next() {
        //    if c == 'l' {
        //        return Config::parse_library
        //}
        //match Some(&*x.to_string()) {
        //    Some("Random") => InitialState::Random,
        //    Some("Blank") => InitialState::Blank(CellState::Dead),

        //    None | Some(&_) => InitialState::Random,
        //}
        InitialState::Random
    }
    pub fn return_options(&self) -> World_Options {
        World_Options {
            width_height: (self.width, self.height),
            init: Config::parse_initial_state(&self.initial_state),
            input_cells: InputCells::Neighbors,
            rules: Rulesets::Conway,
            delay: self.delay,
            alive_char: self.alive_char,
            dead_char: self.dead_char,
        }
    }
}
