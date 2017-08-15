extern crate ncurses;
use ncurses::{initscr, noecho, clear, refresh, printw, endwin, stdscr, getmaxyx};
extern crate argparse;
use argparse::{ArgumentParser, StoreTrue, Store};
extern crate conway;
use conway::world::return_types::{StepError, StepResult};
use conway::world::cell::{Cell, CellState};
use conway::world::builder::InitialState;
use conway::world::builder::designs::Design;
use conway::world::{World, World_Options};
use conway::world::rules::{Ruleset, Rulesets};
use conway::world::rules::input_cells::InputCells;
mod config;
use config::Config;
mod display;
use display::Renderer;
use std::{thread, time, env, process};
use std::io::prelude::*;

/*
 *TODO: Pause, play, step forward, step back, quit
 *TODO: Command line option parser
 *TODO: Config file parser for InputCells, Rulesets, and InitialState
 *TODO: Color options, ANSI codes
 *TODO: Cell state history
 *TODO: Non-square cells
 *TODO: Extensibility
 *TODO: Sub-automata (arbitrary depth?)
*/

fn main() {
    //start ncurses
    initscr();
    noecho();
    //get terminal dimensions:
    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);
    let (max_x, max_y) = (max_x as u64 / 2, max_y as u64);

    let mut config = Config {
        //cli specific options
        alive_char: 'o',
        dead_char: ' ',
        //core options
        output_file: String::new(),
        initial_state: String::from("Random"),
        adjacent_rules: String::from("Neighbors"),
        ruleset: String::from("Conway"),
        delay: 50,
        width: max_x,
        height: max_y,
    };
    {
        //parse command line arguments
        let mut ap = ArgumentParser::new();
        ap.refer(&mut config.alive_char)
            .add_option(&["-c", "--alive-char"], Store,
                        "Alive character");
        ap.refer(&mut config.dead_char)
            .add_option(&["-d", "--dead-char"], Store,
                        "Dead character");
        ap.refer(&mut config.output_file)
            .add_option(&["-o", "--output-file"], Store,
                        "File for debug output");
        ap.refer(&mut config.initial_state)
            .add_option(&["-i", "--init"], Store,
                        "Initial grid state");
        ap.refer(&mut config.adjacent_rules)
            .add_option(&["-a", "--adjacent-rules"], Store,
                        "Input cell pattern");
        ap.refer(&mut config.ruleset)
            .add_option(&["-r", "--ruleset"], Store,
                        "Ruleset");
        ap.refer(&mut config.delay)
            .add_option(&["-d", "--delay"], Store,
                        "Delay between ticks");
        ap.parse_args();
    }
    //println!("--alive-char: {}", config.alive_char);
    //println!("--dead-char: {}", config.dead_char);
    //println!("--debug-output: {}", config.output_file);
    //println!("--initial-state: {}", config.initial_state);
    //println!("--adjacent-rules: {}", config.adjacent_rules);
    //println!("--ruleset: {}", config.ruleset);
    //println!("--delay: {}", config.delay);

    let world_options = config.return_options();

    let mut world = World::new(world_options);

    let render_params = world.return_render_params();

    let renderer = Renderer::new(render_params, world.return_width());

    let run = true;
    while run {
        clear();
        renderer.render(world.return_grid());
        refresh();
        world.step();
    }

    endwin();
}
