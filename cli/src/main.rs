extern crate ncurses;
extern crate midir;
use midir::{MidiInput, MidiOutput, Ignore};
use ncurses::{clear, getch, nodelay, stdscr};
use std::{thread, time, str};
extern crate conway;
extern crate rand;
use rand::Rng;
//use conway::world::return_types::{StepError, StepResult};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use conway::world::{World, WorldOptions};
use conway::world::cell::{Cell, CellState};
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
 *TODO: Restart (rebuild world)
 *TODO: Interactive mode, ncurses mouse control, hjkl control
         * add cells, remove cells
 *TODO: Cell state history + Display visited cells
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
            width: term_width,
            height: term_height,
            init: InitialState::Random,
            input_cells: InputCells::Neighbors,
            rules: Rulesets::Conway,
        },
        render_options: RenderOptions {
            delay: 50,
            width: term_width,
            height: term_height,
            live_char: String::from("o"),
            dead_char: String::from(" "),
            filled: false,
            inverse: false,
            padding: true,
            stopped: false,
            color: Color::White,
            dead_color: Color::Black,
            //?
            time_slice: false,
        }
    };

    let (render_opts, world_opts) = Parser::new(defaults).parse().return_options();
    let mut paused = render_opts.stopped.clone();
    renderer.set_options(render_opts);

    let mut world = World::new(world_opts);
    
    let mut rng = rand::thread_rng();
    //SETUP MIDI STUFF
    let mut midi_in = MidiInput::new("midi_pipe_in").unwrap();
    midi_in.ignore(Ignore::None);
    let mut input_port_number: usize = 5500;

    for i in 0..midi_in.port_count() {
        if midi_in.port_name(i).unwrap().contains("APC Key 25") {
            input_port_number = i;
        }
    }
    let mut ADD_RANDOM_CELLS = Arc::new(AtomicBool::new(true));
    let mut a_r_c_ref1 = Arc::clone(&ADD_RANDOM_CELLS);
    let mut a_r_c_ref2 = Arc::clone(&ADD_RANDOM_CELLS);
    let _conn_in_ = midi_in.connect(input_port_number, "conway-midi-in", move |stamp, message, _| {
        //println!("{}: {:?} (len = {})", stamp, message, message.len());
        a_r_c_ref1.store(true, Ordering::Relaxed);
        //a_r_c_ref1 = Arc::new(AtomicBool::new(true));
    }, ());
    //

    let mut run = true;
    while run {
        let action = process_keyboard_input(getch());

        match action {
            Action::Quit => run = false,
            Action::StartStop => paused = !paused,
            Action::Step => {world.step(); ()},
            Action::None => (),
        }

        renderer.render(world.return_grid());
        //thread::sleep(time::Duration::from_millis(100));

        ////////////////////////////////////////
        //let mut a_r_c: &AtomicBool = &mut ADD_RANDOM_CELLS;
        if a_r_c_ref2.load(Ordering::Relaxed) == true {
            world.add_random_cells(30);
            //a_r_c_ref2 = Arc::new(AtomicBool::new(false));
            a_r_c_ref2.store(false, Ordering::Relaxed);
        }
        //    let grid_len = world.return_grid().len();
        //    let new_cells: Vec<u32>;
        //    for i in 0..30 {
        //        let new_cell = rng.gen_range(0, grid_len);
        //        world.set_cell_state(new_cell, CellState::Live);
        //    }
        //    println!("ADDING RANDOM CELLS");
        //    ADD_RANDOM_CELLS = false;
        //}
    //fn populate_random(grid_ref: &mut Vec<Cell>) {
    //    let mut rng = rand::thread_rng();
    //    for cell in grid_ref.iter_mut() {
    //        match rng.gen() {
    //            true => cell.set_state(CellState::Live),
    //            false => cell.set_state(CellState::Dead),
    //        };
    //    }
    //}
        ////////////////////////////////////////

        match if !paused {
            world.step()
        } else {
            Ok(conway::world::return_types::StepResult {
                steps: 0,
                updated_cells: 0,
            })
        }{
            Ok(stats) => stats, //FIXME:do something with this. And actually populate them!!!
            Err(e) => panic!("Error stepping world forward: {:?}", e),
        };
        //println!("test");
    }

    clear();
    renderer.end();
}
fn process_keyboard_input(c: i32) -> Action {
    if c == 'q' as i32 {
        Action::Quit
    } else if c == ' ' as i32 {
        Action::StartStop
    } else if c == 'n' as i32 {
        Action::Step
    } else {
        Action::None
    }
}
enum Action {
    Quit,
    StartStop,
    Step,
    None,
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
