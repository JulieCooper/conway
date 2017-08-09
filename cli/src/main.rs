extern crate ncurses;
use ncurses::*;
extern crate conway;
use conway::world::types::{StepError, StepResult};
use conway::world::cell::{Cell, CellState};
use conway::world::builder::InitialState;
use conway::world::builder::designs::Design;
use conway::world::{World, Dims, WorldOptions};
use conway::world::rules::*;
use std::{thread, time};

fn main() {
    /* Conway DSL:
     * (Dead, (Live (3 Live)))
     * (Live, (Live (..1 Dead)
     *              (4.. Dead)))
     */
    let conway =
        |state: &CellState, adj_states: &Vec<CellState>| {
            if let &CellState::Dead = state {
                match adj_states.iter().filter(|x| x == &&CellState::Live).count() {
                    3 => CellState::Live,
                    _ => CellState::Dead,
                }
            } else {
                match adj_states.iter().filter(|x| x == &&CellState::Live).count() {
                    0 | 1 => CellState::Dead,
                    2 | 3 => CellState::Live,
                    4...9 => CellState::Dead,
                    _ => CellState::Uninitialized,
                }
            }
        };

    //setup ncurses
    initscr();
    noecho();

    //create ruleset object
    let ruleset = Ruleset {
        input_cells: Input_Cells::Neighbors,
        rules: Box::new(conway),
    };

    let options = WorldOptions {
        width_height: Dims::Auto,
        init: InitialState::Library(Design::LWSS),
        ruleset: Some(ruleset),
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
            row.push('0');
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
