#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum CellState {
    Live,
    Dead,
    Uninitialized,
    OOB,
}
pub struct Cell {
    xy: (u64, u64),
    state: usize,
}
impl Cell {
    pub fn new(xy: (u64, u64), init: usize) -> Self {
        Cell { xy: xy, state: init }
    }
    pub fn set_state(&mut self, new: usize) {
        self.state = new;
    }
    pub fn get_state(&self) -> usize {
        self.state
    }
    pub fn get_xy(&self) -> (u64, u64) {
        self.xy
    }
}
