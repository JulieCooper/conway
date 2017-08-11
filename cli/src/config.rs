use std::env;
use std::error::Error;
//use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    //* array of cellstate display chars
    pub initial_state: String,
    pub input_cell_pattern: String,
    pub ruleset: String,
    pub dimensions: (u64, u64),
    pub delay: u8,
    //pub output-file: String
}
impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        for arg in args.next() {
        }

        Ok(Config {
            initial_state: String::new(),
            input_cell_pattern: String::new(),
            ruleset: String::new(),
            dimensions: (0, 0),
            delay: 50,
        })
    }
}
