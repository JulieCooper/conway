extern crate rand;
use rand::Rng;

#[derive(PartialEq, Clone)]
pub enum CellState {
    Live,
    Dead,
    Other,
    OOB,
}
pub enum Design {
    Horizontal_Lines,
    Vertical_Lines,
}
pub enum InitialState {
    Random,
    Blank,
    Ordered(Vec<(u64, u64)>),
    Preconstructed(Design),
    Lambda,
}

pub struct WorldOptions {
    pub adj_cells: Vec<(i8, i8)>,
    pub rules: Box<FnMut(&CellState, &Vec<CellState>) -> CellState>,
    pub init: InitialState,
}
pub struct Cell {
    xy: (u64, u64),
    pub state: CellState,
}
impl Cell {
    fn new(xy: (u64, u64), init: CellState) -> Self {
        Cell { xy: xy, state: init }
    }
    fn set_state(&mut self, new: CellState) {
        self.state = new;
    }
}
pub enum StepError {
    Reason1,
    Reason2,
    Reason3,
}
pub struct StepResult {
    steps: u64,
    updated_cells: u64,
}
pub struct World {
    time: u64,
    width: u64,
    height: u64,
    grid: Vec<Cell>,
    options: WorldOptions,
}
impl World {
    pub fn new(w: u64, h: u64, options: WorldOptions) -> Self {
        //setup vector
        let mut grid = Vec::with_capacity(w as usize * h as usize);

        //populate grid with Dead cells
        for y in 0..h {
            for x in 0..w {
                let cell = Cell::new((x,y), CellState::Dead);
                grid.push(cell);
            }
        }

        //lambda for generating index from x, y coordinates
        let gen_index = |x, y| -> usize {
            x as usize + y as usize * w as usize
        };
        //match init option and `make it so`
        match options.init {
            InitialState::Random => {
                let mut rng = rand::thread_rng();
                for mut cell in grid.iter_mut() {
                    let rand_state = match rng.gen() {
                        true => cell.set_state(CellState::Live),
                        false => cell.set_state(CellState::Dead),
                    };
                }
            },
            InitialState::Blank => (),
            InitialState::Ordered(ref coords) => {
                for coord in coords.iter() {
                    match grid.get_mut(gen_index(coord.0, coord.1)) {
                        Some(cell) => {
                            cell.set_state(CellState::Live);
                        },
                        None => panic!("invalid init coord"),
                    }
                }
            },
            InitialState::Preconstructed(ref design) => {
                match design {
                    &Design::Horizontal_Lines => {
                        for x in 0..w {
                            for y in 0..h {
                                if y % 2 == 0 {
                                    match grid.get_mut(gen_index(x, y)) {
                                        Some(cell) => {
                                            cell.set_state(CellState::Live);
                                        },
                                        None => panic!("invalid preconstructed coord"),
                                    }
                                }
                            }
                        }
                    },
                    &Design::Vertical_Lines => {
                    },
                }
            },
            InitialState::Lambda => (),
        }

        World { grid: grid, width: w, height: h, time: 0, options: options }
    }

    fn set_cell_state(&mut self, index: usize, new: CellState) {
        if let Some(cell) = self.grid.get_mut(index) {
            cell.set_state(new)
        }
    }

    fn get_cell_state(&self, xy: (u64, u64)) -> CellState {
        let index = xy.0 + self.width * xy.1;

        match self.grid.get(index as usize) {
            Some(cell) => cell.state.clone(),
            None => CellState::OOB,
        }
    }

    fn get_neighbor_states(&self, xy: (u64, u64)) -> Vec<CellState> {
        let (x, y) = xy;
        self.options.adj_cells.iter().map(|rule| {
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
            neighbor_states.push(self.get_neighbor_states(cell.xy));
        }
        let mut processed = Vec::new();
        for (index, cell) in self.grid.iter().enumerate() {
            if let Some(states) = neighbor_states.get(index) {
                processed.push(
                    (self.options.rules)(&cell.state, states)
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
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
