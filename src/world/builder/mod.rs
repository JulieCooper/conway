pub mod designs;
use self::designs::Design;
use world::{Cell, CellState};
extern crate rand;
use rand::Rng;

pub enum InitialState {
    Blank(CellState),
    Random,
    Ordered(Vec<(u64, u64)>),
    Library(Design),
    Lambda,
}
pub struct WorldBuilder {
    width: u64,
    height: u64,
    init: InitialState,
}
impl WorldBuilder {
    pub fn new(width: u64, height: u64, init: InitialState) -> Self {
        WorldBuilder { 
            width: width,
            height: height,
            init: init,
        }
    }
    fn gen_index(&self, x: u64, y: u64) -> usize {
        x as usize + y as usize * self.width as usize
    }
    fn add_cells(&self, grid_ref: &mut Vec<Cell>, w: u64, h: u64) {
        //fill grid by x and y with new cells
        for y in 0..h {
            for x in 0..w {
                let cell = Cell::new((x,y), CellState::Uninitialized);
                grid_ref.push(cell);
            }
        }
    }
    pub fn build(&self, grid_ref: &mut Vec<Cell>) {
        self.add_cells(grid_ref, self.width, self.height);

        //match InitialState and make it so
        match self.init {
            InitialState::Blank(ref state) => WorldBuilder::populate_all(grid_ref, state.clone()),
            InitialState::Random => WorldBuilder::populate_random(grid_ref),
            InitialState::Ordered(ref coords) => {
                self.populate_ordered(grid_ref, coords);
            },
            InitialState::Library(ref design) => {
                self.populate_library(grid_ref, design);
            },
            //InitialState::Lambda(f) => WorldBuilder::populate_lambda(grid_ref, f),
            InitialState::Lambda => (),
        }
    }
    fn populate_all(grid_ref: &mut Vec<Cell>, state: CellState) {
        for cell in grid_ref.iter_mut() {
            cell.set_state(state.clone());
        }
    }
    fn populate_random(grid_ref: &mut Vec<Cell>) {
        let mut rng = rand::thread_rng();
        for cell in grid_ref.iter_mut() {
            match rng.gen() {
                true => cell.set_state(CellState::Live),
                false => cell.set_state(CellState::Dead),
            };
        }
    }
    fn populate_ordered(&self, grid_ref: &mut Vec<Cell>, coords: &Vec<(u64, u64)>) {
        WorldBuilder::populate_all(grid_ref, CellState::Dead);
        for coord in coords.iter() {
            match grid_ref.get_mut(self.gen_index(coord.0, coord.1)) {
                Some(cell) => {
                    cell.set_state(CellState::Live);
                },
                None => panic!("invalid init coord"),
            }
        }
    }
    fn populate_library(&self, grid_ref: &mut Vec<Cell>, design: &Design) {
        let data = design.get_data();
        self.populate_ordered(grid_ref, &data);
    }
    fn populate_lambda(grid_ref: &mut Vec<Cell>, mut f: Box<FnMut(&Cell)->CellState>) {
        for cell in grid_ref.iter_mut() {
            let new_state = f(cell);
            cell.set_state(new_state);
        }
    }
}
