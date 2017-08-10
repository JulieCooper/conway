extern crate ncurses;
use ncurses::*;
extern crate conway;
use conway::world::types::{StepError, StepResult};
use conway::world::cell::{Cell, CellState};
use conway::world::builder::InitialState;
use conway::world::builder::designs::Design;
use conway::world::{World, Dims, WorldOptions};
use conway::world::rules::{Ruleset, Rulesets};
use conway::world::rules::input_cells::Input_Cells;
use std::{thread, time};

fn main() {
    //setup ncurses
    initscr();
    noecho();

    let options = WorldOptions {
        width_height: Dims::Auto,
        init: InitialState::Random,//Library(Design::Eureka),
        input_cells: Input_Cells::Neighbors,
        rules: Rulesets::Conway,
    };

    //create world
    let mut world = World::new(options);

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
