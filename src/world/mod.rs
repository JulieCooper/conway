pub mod cell;
use self::cell::{Cell, CellState};
pub mod builder;
use self::builder::{InitialState, WorldBuilder};
pub mod return_types;
use self::return_types::{StepResult, StepError};
pub mod rules;
use self::rules::{DSLRuleset, Rulesets};
use self::rules::input_cells::InputCells;

#[derive(Clone)]
pub struct WorldOptions {
    pub width: usize,
    pub height: usize,
    pub init: InitialState,
    pub input_cells: InputCells,
    pub rules: Rulesets,
}
impl WorldOptions {
    pub fn new() -> Self {
        WorldOptions {
            width: 0,
            height: 0,
            init: InitialState::Random,
            input_cells: InputCells::Neighbors,
            rules: Rulesets::Conway,
        }
    }
}

pub struct World {
    time: u64,
    width: usize,
    height: usize,
    grid: Vec<Cell>,
    input_cells: InputCells,
    rules: DSLRuleset,
}
impl World {
    pub fn new(options: WorldOptions) -> Self {
        let (w, h) = (options.width, options.height);

        let mut grid = Vec::with_capacity(w as usize * h as usize);

        let wb = WorldBuilder::new(w, h, options.init);
        //FIXME: worldbuilder should NOT take init as parameter,
        //init should be passed in to build() after world builder
        //is created.
        
        //build world
        wb.build(&mut grid);

        let rules = options.rules.get_data();

        World {
            time: 0,
            width: w, height: h,
            grid: grid,
            input_cells: options.input_cells,
            rules: rules,
        }
    }

    fn set_cell_state(&mut self, index: usize, new: CellState) {
        if let Some(cell) = self.grid.get_mut(index) {
            cell.set_state(new)
        }
    }

    fn get_cell_state(&self, xy: (u64, u64)) -> CellState {
        let index = xy.0 + self.width as u64 * xy.1;

        match self.grid.get(index as usize) {
            Some(cell) => cell.get_state().clone(),
            None => CellState::OOB,
        }
    }

    fn get_neighbor_states(&self, xy: (u64, u64)) -> Vec<(CellState, usize)> {
        let (x, y) = xy;
        //get neighbor states
        let neighbor_states = self.input_cells.get_data().iter().map(|rule| {
            let x = x as i64 + rule.0 as i64;
            let y = y as i64 + rule.1 as i64;
            if (x < 0) | (y < 0) | (x > self.width as i64) | (y > self.height as i64) {
                CellState::OOB
            } else {
                let coords = (x as u64, y as u64);
                self.get_cell_state(coords)
            }
        }).collect::<Vec<_>>();

        //format neighbor states for input to ruleset (Amount, ResultState)
        let mut output_vector = Vec::new();
        use self::cell::CellState::{Dead, Live};
        for state in vec![Dead, Live].iter() {
            let num = neighbor_states.iter().filter(|x| x == &state).count();
            output_vector.push( (state.clone(), num) );
        }

        output_vector
    }

    //map through all cells, applying rules and returning computed next grid state
    fn process_cells(&mut self) -> Vec<CellState> {

        let mut neighbor_states = Vec::new();
        for cell in self.grid.iter() {
            neighbor_states.push(self.get_neighbor_states(cell.get_xy()));
        }
        let mut processed = Vec::new();
        for (index, cell) in self.grid.iter().enumerate() {
            if let Some(nb_states) = neighbor_states.get(index) {
                processed.push(
                    self.rules.compute(cell.get_state().clone(), nb_states.clone())
                );
            }
        }

        processed
    }

    //appy changes produced by process_cells() and return statistics
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
        self.time = self.time + 1;
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
