extern crate ncurses;
use ncurses::{initscr, noecho, endwin, stdscr, getmaxyx, start_color};
extern crate conway;
//use conway::world::return_types::{StepError, StepResult};
use conway::world::World;
mod config;
use config::Config;
mod parser;
use parser::Parser;
mod display;
use display::Renderer;
//use std::io::prelude::*;

/*
 *TODO: Config file parser for InputCells, Rulesets, and InitialState
 *TODO: Pause, play, step forward, step back, quit
 *TODO: Interactive mode
 *TODO: Cell state history
 *TODO: Hex cells.. Non-square cells
 *TODO: Extensibility
 *TODO: Sub-automata (arbitrary depth?)
*/

fn main() {
    let mut renderer = Renderer::new();
    renderer.initialize();
    let (term_width, term_height) = renderer.return_term_size();
    //let mut world = World::new();
    let defaults = Config {
        live_char: 'o',
        dead_char: ' ',
        filled: false,
        inverse: false,
        padding: true,
        color: String::from("White"),
        dead_color: String::from("Black"),
        //?
        time_slice: false,
        //
        output_file: String::new(),
        initial_state: String::from("Random"),
        adjacent_rules: String::from("Neighbors"),
        ruleset: String::from("Conway"),
        delay: 50,
        width: term_width,
        height: term_height,
    };

    let (render_opts, world_opts) = Parser::new(defaults).parse().return_options();
    renderer.set_options(render_opts);

    let mut world = World::new(world_opts);

    let run = true;
    while run {
        renderer.render(world.return_grid());
        match world.step() {
            Ok(stats) => stats, //FIXME:do something with this. And actually populate them!!!
            Err(e) => panic!("Error stepping world forward: {:?}", e),
        };
    }

    renderer.end();
}
