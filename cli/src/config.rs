extern crate argparse;
use argparse::{ArgumentParser, StoreTrue, Store};
use conway::world::World_Options;
use conway::world::cell::CellState;
use conway::world::builder::InitialState;
use conway::world::builder::designs::Design;
use conway::world::rules::Rulesets;
use conway::world::rules::input_cells::InputCells;
use std::env;
use std::error::Error;
//use std::fs::File;
use std::io::prelude::*;

#[derive(Clone)]
pub enum Color {
    Red,
    Green,
    Blue,
    White,
}
pub struct Render_Options {
    pub width: usize,
    pub height: u64,
    pub delay: u64,
    pub live_char: char,
    pub dead_char: char,
    pub filled: bool,
    pub color: Color,
}
//what is recieved from command line args
pub struct Config {
    pub live_char: char,
    pub dead_char: char,
    pub filled: bool,
    pub color: String,
    //
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
        match Some(&*x.to_string()) {
            Some("Random") => InitialState::Random,
            Some("Blank") => InitialState::Blank(CellState::Dead),
            string @ Some(&_) => {
                match string {
                    Some("Block") => InitialState::Library(Design::Block),
                    Some("Beehive") => InitialState::Library(Design::Beehive),
                    Some("Loaf") => InitialState::Library(Design::Loaf),
                    Some("Boat") => InitialState::Library(Design::Boat),
                    Some("Tub") => InitialState::Library(Design::Tub),
                    Some("Blinker") => InitialState::Library(Design::Blinker),
                    Some("Toad") => InitialState::Library(Design::Toad),
                    Some("Beacon") => InitialState::Library(Design::Beacon),
                    Some("Pulsar") => InitialState::Library(Design::Pulsar),
                    Some("Eureka") => InitialState::Library(Design::Eureka),
                    Some("Glider") => InitialState::Library(Design::Glider),
                    Some("LWSS") => InitialState::Library(Design::LWSS),
                    Some(&_) => InitialState::Random,
                    None => InitialState::Random,
                }
            },
            None => InitialState::Random,
        }
    }
    fn parse_ruleset(x: &String) -> Rulesets {
        match Some(&*x.to_string()) {
            Some("Conway") => Rulesets::Conway,
            Some("ConwayEasy") => Rulesets::ConwayEasy,
            Some(&_) => Rulesets::Conway,
            None => Rulesets::Conway,
        }
    }
    fn parse_color(x: &String) -> Color {
        match Some(&*x.to_string()) {
            Some("Red") => Color::Red,
            Some("Green") => Color::Green,
            Some("Blue") => Color::Blue,
            Some(&_) => Color::White,
            None => Color::White,
        }
    }
    pub fn return_options(&self) -> (Render_Options, World_Options) {
        (
            Render_Options {
                width: self.width as usize,
                height: self.height,
                delay: self.delay,
                live_char: self.live_char,
                dead_char: self.dead_char,
                filled: self.filled,
                color: Config::parse_color(&self.color),
            },
            World_Options {
                width_height: (self.width, self.height),
                init: Config::parse_initial_state(&self.initial_state),
                input_cells: InputCells::Neighbors,
                rules: Config::parse_ruleset(&self.ruleset),//Rulesets::Conway,
            }
        )
    }
}
