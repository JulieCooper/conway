extern crate ncurses;
extern crate conway;
//use conway::world::return_types::{StepError, StepResult};
use conway::world::{World, WorldOptions};
use conway::world::rules::Rulesets;
use conway::world::rules::input_cells::InputCells;
use conway::world::builder::InitialState;
mod parser;
use parser::Parser;
mod renderer;
use renderer::{Renderer, RenderOptions, Color};
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
        world_options: WorldOptions {
            width_height: (term_width, term_height),
            init: InitialState::Random,
            input_cells: InputCells::Neighbors,
            rules: Rulesets::Conway,
        },
        render_options: RenderOptions {
            delay: 50,
            width: term_width,
            height: term_height,
            live_char: 'o',
            dead_char: ' ',
            filled: false,
            inverse: false,
            padding: true,
            color: Color::White,
            dead_color: Color::Black,
            //?
            time_slice: false,
        }
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

#[derive(Clone)]
pub struct Config {
    pub world_options: WorldOptions,
    pub render_options: RenderOptions,
}
impl Config {
    pub fn return_options(&self) -> (RenderOptions, WorldOptions) {
        (self.render_options.clone(), self.world_options.clone())
    }
}
