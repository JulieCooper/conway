pub mod cell;
use self::cell::{Cell, CellState};
pub mod builder;
use self::builder::{InitialState, WorldBuilder};
use self::builder::designs::*;
pub mod types;
use self::types::{StepResult, StepError};
pub mod rules;
use self::rules::*;
extern crate ncurses;
use world::ncurses::stdscr;
use world::ncurses::getmaxyx;

pub enum Dims {
    Custom(u64, u64),
    Auto,
}
pub struct WorldOptions {
    pub width_height: Dims,
    pub init: InitialState,
    pub ruleset: Option<Ruleset>,
}
pub struct World {
    time: u64,
    width: u64,
    height: u64,
    grid: Vec<Cell>,
    ruleset: Ruleset,
}
impl World {
    pub fn new(options: WorldOptions) -> Self {
        let (w, h) = match options.width_height {
            Dims::Custom(w, h) => (w, h),
            Dims::Auto => {
                let mut max_x = 0;
                let mut max_y = 0;
                getmaxyx(stdscr(), &mut max_y, &mut max_x);

                (max_x as u64 / 2, max_y as u64)
            },
        };

        //setup vector
        let mut grid = Vec::with_capacity(w as usize * h as usize);

        //create WorldBuilder object
        let mut wb = WorldBuilder::new(w, h, options.init);
        
        //build world
        wb.build(&mut grid);

        let ruleset = options.ruleset;

        World {
            time: 0,
            width: w, height: h,
            grid: grid,
            ruleset: ruleset.unwrap(),
        }
    }

    fn set_cell_state(&mut self, index: usize, new: CellState) {
        if let Some(cell) = self.grid.get_mut(index) {
            cell.set_state(new)
        }
    }

    fn get_cell_state(&self, xy: (u64, u64)) -> CellState {
        let index = xy.0 + self.width * xy.1;

        match self.grid.get(index as usize) {
            Some(cell) => cell.get_state().clone(),
            None => CellState::OOB,
        }
    }

    fn get_neighbor_states(&self, xy: (u64, u64)) -> Vec<CellState> {
        let (x, y) = xy;
        self.ruleset.input_cells.get_data().iter().map(|rule| {
            let x = x as i64 + rule.0 as i64;
            let y = y as i64 + rule.1 as i64;
            if (x < 0) | (y < 0) {
                CellState::OOB
            } else {
                let coords = (x as u64, y as u64);
                self.get_cell_state(coords)
            }
        }).collect::<Vec<_>>()
    }

    //map through all cells, applying rules and returning computed next grid state
    fn process_cells(&mut self) -> Vec<CellState> {

        let mut neighbor_states = Vec::new();
        for cell in self.grid.iter() {
            neighbor_states.push(self.get_neighbor_states(cell.get_xy()));
        }
        let mut processed = Vec::new();
        for (index, cell) in self.grid.iter().enumerate() {
            if let Some(states) = neighbor_states.get(index) {
                processed.push(
                    (self.ruleset.rules)(&cell.get_state(), states)
                );
            }
        }

        processed
    }

    fn apply_state_changes(&mut self, new_state: Vec<CellState>) -> StepResult {
        for (index, new) in new_state.into_iter().enumerate() {
            self.set_cell_state(index, new);
        }
        StepResult {
            steps: 10,
            updated_cells: 100,
        }
    }
    pub fn step(&mut self) -> Result<StepResult, StepError> {
        //get list of state changes according to rules
        let changes = self.process_cells();

        //apply state changes to grid and return step statistics
        let sr = self.apply_state_changes(changes);

        Ok(sr)
    }
    pub fn return_grid(&self) -> &Vec<Cell> {
        &self.grid
    }
    pub fn return_width(&self) -> usize {
        self.width as usize
    }
}
