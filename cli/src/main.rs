extern crate ncurses;
use ncurses::{initscr, noecho, endwin, stdscr, getmaxyx, start_color, init_pair};
use ncurses::{COLOR_BLACK};
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
use config::{Config, Color};
mod display;
use display::Renderer;
use std::{thread, time, env, process};
use std::io::prelude::*;

/*
 *TODO: Color options, ANSI codes
 *TODO: Interactive mode
 *TODO: Pause, play, step forward, step back, quit
 *TODO: Config file parser for InputCells, Rulesets, and InitialState
 *TODO: Cell state history
 *TODO: Non-square cells
 *TODO: Extensibility
 *TODO: Sub-automata (arbitrary depth?)
*/

fn main() {
    //start ncurses
    initscr();
    start_color();
    init_pair(1, COLOR_BLACK, COLOR_BLACK);
    noecho();
    //get terminal dimensions:
    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);
    let (max_x, max_y) = (max_x as u64 / 2, max_y as u64);

    let mut config = Config {
        live_char: 'o',
        dead_char: ' ',
        filled: false,
        color: String::from("Green"),
        //
        output_file: String::new(),
        initial_state: String::from("Random"),
        adjacent_rules: String::from("Neighbors"),
        ruleset: String::from("Conway"),
        delay: 50,
        width: max_x,
        height: max_y,
    };
    {
        //parse arguments
        let mut ap = ArgumentParser::new();
        ap.refer(&mut config.live_char)
            .add_option(&["-l", "--live-char"], Store,
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
        ap.refer(&mut config.width)
            .add_option(&["-w", "--width"], Store,
                        "Width of grid in cells");
        ap.refer(&mut config.filled)
            .add_option(&["-f", "--filled"], StoreTrue,
                        "Fill cells instead of printing character");
        ap.refer(&mut config.color)
            .add_option(&["-c", "--color"], Store,
                        "Color for live cells");
        ap.refer(&mut config.height)
            .add_option(&["-h", "--height"], Store,
                        "Height of grid in cells");
        ap.parse_args();
    }

    let (render_ops, world_ops) = config.return_options();

    let mut world = World::new(world_ops);

    let renderer = Renderer::new(render_ops, world.return_width());

    let run = true;
    while run {
        renderer.render(world.return_grid());
        world.step();
    }

    endwin();
}
