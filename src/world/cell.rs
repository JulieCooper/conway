#[derive(PartialEq, Clone)]
pub enum CellState {
    Live,
    Dead,
    Other,
    OOB,
}
pub struct Cell {
    xy: (u64, u64),
    state: CellState,
}
impl Cell {
    pub fn new(xy: (u64, u64), init: CellState) -> Self {
        Cell { xy: xy, state: init }
    }
    pub fn set_state(&mut self, new: CellState) {
        self.state = new;
    }
    pub fn get_state(&self) -> &CellState {
        &self.state
    }
    pub fn get_xy(&self) -> (u64, u64) {
        self.xy
    }
}
