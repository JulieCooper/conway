use conway::world::World_Options;
use conway::world::cell::CellState;
use conway::world::builder::InitialState;
use conway::world::builder::designs::Design;
use conway::world::rules::Rulesets;
use conway::world::rules::input_cells::InputCells;
//use std::error::Error;

#[derive(Clone)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}
pub struct RenderOptions {
    pub width: usize,
    pub height: usize,
    pub delay: u64,
    pub live_char: char,
    pub dead_char: char,
    pub filled: bool,
    pub inverse: bool,
    pub padding: bool,
    pub color: Color,
    pub dead_color: Color,
    //?
    pub time_slice: bool,
}
impl RenderOptions {
    pub fn new() -> Self {
        RenderOptions {
            width: 0,
            height: 0,
            delay: 0,
            live_char: ' ',
            dead_char: ' ',
            filled: false,
            inverse: false,
            padding: false,
            color: Color::White,
            dead_color: Color::White,
            time_slice: false,
        }
    }
}
#[derive(Clone)]
pub struct Config {
    pub live_char: char,
    pub dead_char: char,
    pub filled: bool,
    pub inverse: bool,
    pub padding: bool,
    pub color: String,
    pub dead_color: String,
    //?
    pub time_slice: bool,
    //
    pub output_file: String,
    pub initial_state: String,
    pub adjacent_rules: String,
    pub ruleset: String,
    pub delay: u64,
    pub width: usize,
    pub height: usize,
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
            Some("Black") => Color::Black,
            Some("Red") => Color::Red,
            Some("Green") => Color::Green,
            Some("Yellow") => Color::Yellow,
            Some("Blue") => Color::Blue,
            Some("Magenta") => Color::Magenta,
            Some("Cyan") => Color::Cyan,
            Some("White") => Color::White,
            Some(&_) => Color::White,
            None => Color::White,
        }
    }
    pub fn return_options(&self) -> (RenderOptions, World_Options) {
        (
            RenderOptions {
                width: self.width,
                height: self.height,
                delay: self.delay,
                live_char: self.live_char,
                dead_char: self.dead_char,
                filled: self.filled,
                inverse: self.inverse,
                padding: self.padding,
                time_slice: self.time_slice,
                color: Config::parse_color(&self.color),
                dead_color: Config::parse_color(&self.dead_color),
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
