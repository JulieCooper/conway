extern crate ncurses;
use ncurses::{initscr, noecho, clear, refresh, printw, endwin};
extern crate conway;
use conway::world::return_types::{StepError, StepResult};
use conway::world::cell::{Cell, CellState};
use conway::world::builder::InitialState;
use conway::world::builder::designs::Design;
use conway::world::{World, Dimensions, World_Options};
use conway::world::rules::{Ruleset, Rulesets};
use conway::world::rules::input_cells::InputCells;
use std::{thread, time, env, process};
use std::io::prelude::*;
mod config;
use config::Config;

fn main() {
    //prepare error stream
    let mut stderr = std::io::stderr();

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        writeln!(
            &mut stderr,
            "Problem {}",
            err
        ).expect("Could not write to stderr");
        process::exit(1);
    });

    //setup ncurses
    initscr();
    noecho();

    let world_options = World_Options {
        width_height: Dimensions::Auto,
        init: InitialState::Random,//Library(Design::Eureka),
        input_cells: InputCells::Neighbors,
        rules: Rulesets::Conway,
    };

    //create world
    let mut world = World::new(world_options);

    //main loop
    let run = true;
    while run {
        clear();
        display_grid(world.return_grid(), world.return_width());
        refresh();
        world.step();
        thread::sleep(time::Duration::from_millis(50));
    }

    //cleanup
    endwin();
}

fn display_grid(grid: &Vec<Cell>, width: usize) {
    let mut row = String::new();
    for (index, cell) in grid.iter().enumerate() {
        if let &CellState::Dead = cell.get_state() {
            row.push(' ');
        } else {
            row.push('o');
        }

        if index % width == width - 1 {
            row.push('\n');
            printw(&row.to_string());
            row.clear();
        } else {
            row.push(' ');
        }
    }
}
