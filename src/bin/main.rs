extern crate conway;
use conway::World;
use conway::{Cell, CellState, Ruleset};
use std::{thread, time};

fn main() {
    let init = vec![
        //(2, 1), (1, 3), (2, 3), (3, 3), (3, 2)
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
    let ruleset = Ruleset {
        neighbors: neighbors,
        rules: Box::new(rules),
        init: init,
    };

    let width = 100;
    let mut world = World::new(width, 20, ruleset);

    let run = true;
    while run {
        display_grid(world.return_grid(), width as usize);
        world.step();
        thread::sleep(time::Duration::from_millis(200));
    }
}

fn display_grid(grid: &Vec<Cell>, width: usize) {
    let mut row = String::new();
    println!("-----");
    for (index, cell) in grid.iter().enumerate() {
        if let CellState::Dead = cell.state {
            row.push(' ');
        } else {
            row.push('x');
        }
        row.push(' ');

        if index % width == width - 1 {
            println!("{}", row);
            row.clear();
        }
    }
}
