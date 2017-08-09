extern crate ncurses;
use ncurses::*;
extern crate conway;
use conway::world::types::{StepError, StepResult};
use conway::world::cell::{Cell, CellState};
use conway::world::builder::{InitialState};
use conway::world::{World, WorldOptions};
use std::{thread, time};

fn main() {
    let mut init = vec![
        (2, 1), (1, 3), (2, 3), (3, 3), (3, 2)
    ];
    let adjacent = vec![
        (-1,-1), (0,-1), (1,-1),
        (-1, 0),         (1, 0),
        (-1, 1), (0, 1), (1, 1)
    ];
    let nodiag = vec![
                 (0,-1),        
        (-1, 0),         (1, 0),
                 (0, 1),       
    ];
    let diagonly = vec![
        (-1,-1),         (1,-1),
                                
        (-1, 1),         (1, 1)
    ];
    let farther = vec![
        (-2,-2), (-1,-2), (0,-2), (1,-2), (2,-2),
        (-2,-1),                          (2,-1),
        (-2, 0),                          (2, 0),
        (-2, 1),                          (2, 1),
        (-2, 2), (-1, 2), (0, 2), (1, 2), (2, 2)
    ];
    let corners = vec![
        (-2,-2), (-1,-2),         (1,-2), (2,-2),
        (-2,-1),                          (2,-1),

        (-2, 1),                          (2, 1),
        (-2, 2), (-1, 2),         (1, 2), (2, 2)
    ];
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
                    _ => CellState::Other,
                }
            }
        };
    let options = WorldOptions {
        input_cells: adjacent,
        rules: Box::new(conway),
        init: InitialState::Random,
    };

    //setup ncurses, get window dimensions
    initscr();
    noecho();
    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);
    let width = (max_x as u64) / 2;
    let height = max_y as u64;

    //create world
    let mut world = World::new(width, height, options);

    //main loop
    let run = true;
    while run {
        clear();
        display_grid(world.return_grid(), width as usize);
        refresh();
        world.step();
        thread::sleep(time::Duration::from_millis(100));
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
            row.push('*');
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
