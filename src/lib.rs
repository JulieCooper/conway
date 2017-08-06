#[derive(PartialEq, Clone)]
enum CellState {
    Live,
    Dead,
    Other,
    OOB,
}
struct Cell {
    xy: (u64, u64),
    state: CellState,
}
impl Cell {
    fn new(xy: (u64, u64), init: CellState) -> Self {
        Cell { xy: xy, state: init }
    }
    fn set_state(&mut self, new: CellState) {
        self.state = new;
    }
}
pub struct World {
    steps: u64,
    width: u64,
    height: u64,
    grid: Vec<Cell>,
}
pub enum StepError {
    Reason1,
    Reason2,
    Reason3,
}
struct CellTransition {
    xy: (u64, u64),
    old: CellState,
    new: CellState,
}
pub struct StepResult {
    steps: u64,
    updated_cells: u64,
}
impl World {
    pub fn new(w: u64, h: u64/*,options: WorldOptions*/) -> Self {

        //setup vector
        let mut grid = Vec::with_capacity(w as usize * h as usize);

        //populate grid
        for y in 0..h {
            for x in 0..w {
                grid.push(Cell::new((x,y), CellState::Dead));
            }
        }

        World { grid: grid, width: w, height: h, steps: 0 }
    }
    pub fn set_rules<F: FnMut(u64)>(&mut self, f: F) {}
    fn set_cell_state(&mut self, xy: (u64, u64), new: CellState) {
        let index = xy.0 + (self.width - 1) * xy.1;

        if let Some(cell) = self.grid.get_mut(index as usize) {
            cell.set_state(new)
        }
    }
    fn get_cell_state(&self, xy: (u64, u64)) -> CellState {
        let index = xy.0 + (self.width - 1) * xy.1;

        match self.grid.get(index as usize) {
            Some(cell) => cell.state.clone(),
            None => CellState::OOB,
        }
    }
    //map through all cells, applying ruleset and returning computed next grid state
    fn process_cells(&self) -> Vec<CellTransition> {
        //closure to apply to each cell to determine neighboring cell states
        let neighbors = |xy: (u64, u64)| {
            let adjacent: Vec<(i8, i8)> = vec![
                (-1,-1), (0,-1), (1,-1),
                (-1, 0),         (1, 0),
                (-1, 1), (0, 1), (1, 1)
            ];
            let neighbor_states = adjacent.into_iter().map(|adj_rules| {
                let x = xy.0 as i64 + adj_rules.0 as i64;
                let y = xy.1 as i64 + adj_rules.1 as i64;
                if (x < 0) | (y < 0) {
                    CellState::OOB
                } else {
                    let coords = (x as u64, y as u64);
                    self.get_cell_state(coords)
                }
            }).collect::<Vec<_>>();

            neighbor_states
        };
        let closure = |nb| {
            CellTransition { xy: (0, 0), old: CellState::Live, new: CellState::Dead, }
        };
        let transitions = self.grid.iter().map(|cell| {
            closure(neighbors(cell.xy))
        }).collect::<Vec<_>>();

        transitions
    }
    /*
     * How to keep track of states, their behaviors, and transitions between states?
     */
    fn apply_state_changes(&mut self, changes: Vec<CellTransition>) -> StepResult {
        changes.into_iter().map(|transition| {
            let CellTransition { xy, old, new } = transition;
            self.set_cell_state(xy, new);
        });
        StepResult {
            steps: 10,
            updated_cells: 100,
        }
    }
    pub fn step(&mut self) -> Result<StepResult, StepError> {
        //get list of state changes according to ruleset
        let changes = self.process_cells(/*lambda to describe ruleset*/);

        //apply state changes to grid and return step statistics
        let sr = self.apply_state_changes(changes);

        Ok(sr)
    }
    fn return_grid(&self) -> &Vec<Cell> {
        &self.grid
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
