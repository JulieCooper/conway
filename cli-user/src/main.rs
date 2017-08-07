extern crate ncurses;
use ncurses::*;
extern crate conway;
use conway::World;
use conway::{Cell, CellState, WorldOptions, InitialState, Design};
use std::{thread, time};

fn main() {
    let mut init = vec![
        (2, 1), (1, 3), (2, 3), (3, 3), (3, 2)
    ];
    let neighbors = vec![
        (-1,-1), (0,-1), (1,-1),
        (-1, 0),         (1, 0),
        (-1, 1), (0, 1), (1, 1)
    ];
    let rules =
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
        adj_cells: neighbors,
        rules: Box::new(rules),
        init: InitialState::Random,
    };

    /* Start ncurses. */
    initscr();
    noecho();

    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);
    let width = max_x as u64 / 2;
    let height = max_y as u64;

    let mut world = World::new(width, height, options);




    let run = true;
    while run {
        clear();
        display_grid(world.return_grid(), max_x as usize);
        refresh();
        world.step();
        thread::sleep(time::Duration::from_millis(100));
    }

    endwin();
}

fn display_grid(grid: &Vec<Cell>, width: usize) {
    let mut row = String::new();
    for (index, cell) in grid.iter().enumerate() {
        if let CellState::Dead = cell.state {
            row.push(' ');
        } else {
            row.push('o');
        }
        row.push(' ');

        if index % width == width - 1 {
            printw(&row.to_string());
            row.clear();
        }
    }
}
