extern crate conway;
use conway::World;
use conway::{Cell, CellState, Ruleset};
use std::{thread, time};

fn main() {
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
        rules: Box::new(rules)
    };

    let mut world = World::new(50, 10, ruleset);

    let run = true;
    //world.step();
    //let grid = world.return_grid();
    //display_grid(grid);
    while run {
        display_grid(world.return_grid());
        world.step();
        thread::sleep(time::Duration::from_secs(1));
    }
}

fn display_grid(grid: &Vec<Cell>) {
    let mut row = String::new();
    println!("-----");
    for (index, cell) in grid.iter().enumerate() {
        if let CellState::Dead = cell.state {
            row.push(' ');
        } else {
            row.push('*');
        }
        row.push(' ');

        if index % 50 == 49 {
            println!("{}", row);
            row.clear();
        }
    }
}
