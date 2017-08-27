pub mod designs;
use self::designs::Design;
use world::{Cell, CellState};
extern crate rand;
use std::str::FromStr;
use rand::Rng;

//TODO: Generalize procedural starting grid states
//TODO: Center prefabs
#[derive(Clone)]
pub enum InitialState {
    Blank(CellState),
    Random,
    Ordered(Vec<(u64, u64, usize)>),
    Library(Design),
    Lambda,
}
impl FromStr for InitialState {
    type Err = ();
    fn from_str(s: &str) -> Result<InitialState, ()> {
        match s {
            //procedural
            "Random" => Ok(InitialState::Random),
            "Blank" => Ok(InitialState::Blank(CellState::Dead)),
            "Filled" => Ok(InitialState::Blank(CellState::Live)),
            //prefab
            "WireTest" => Ok(InitialState::Library(Design::WireTest)),
            _ => Err(()),
        }
    }
}
pub struct WorldBuilder {
    width: usize,
    height: usize,
    init: InitialState,
}
impl WorldBuilder {
    pub fn new(width: usize, height: usize, init: InitialState) -> Self {
        WorldBuilder { 
            width: width,
            height: height,
            init: init,
        }
    }
    fn gen_index(&self, x: u64, y: u64) -> usize {
        x as usize + y as usize * self.width as usize
    }
    fn add_cells(&self, grid_ref: &mut Vec<Cell>, w: usize, h: usize) {
        //fill grid by x and y with new cells
        for y in 0..h {
            for x in 0..w {
                let cell = Cell::new((x as u64,y as u64), 9999);
                grid_ref.push(cell);
            }
        }
    }
    pub fn build(&self, grid_ref: &mut Vec<Cell>) {
        self.add_cells(grid_ref, self.width, self.height);

        //match InitialState and make it so
        match self.init {
            InitialState::Blank(ref state) => WorldBuilder::populate_all(grid_ref, 0),
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
    fn populate_all(grid_ref: &mut Vec<Cell>, state: usize) {
        for cell in grid_ref.iter_mut() {
            cell.set_state(state);
        }
    }
    fn populate_random(grid_ref: &mut Vec<Cell>) {
        let mut rng = rand::thread_rng();
        for cell in grid_ref.iter_mut() {
           let state =  rng.gen_range(0, 1000);
           let state = match state {
               0...2 => 1,
               1...400 => 3,
               _ => 0,
           };
           cell.set_state(state);
        }
    }
    fn populate_ordered(&self, grid_ref: &mut Vec<Cell>, coords: &Vec<(u64, u64, usize)>) {
        WorldBuilder::populate_all(grid_ref, 0);
        for coord in coords.iter() {
            match grid_ref.get_mut(self.gen_index(coord.0, coord.1)) {
                Some(cell) => {
                    cell.set_state(coord.2);
                },
                None => panic!("invalid init coord"),
            }
        }
    }
    fn populate_library(&self, grid_ref: &mut Vec<Cell>, design: &Design) {
        let data = design.get_data();
        self.populate_ordered(grid_ref, &data);
    }
    //fn populate_lambda(grid_ref: &mut Vec<Cell>, mut f: Box<FnMut(&Cell)->CellState>) {
    //    for cell in grid_ref.iter_mut() {
    //        let new_state = f(cell);
    //        cell.set_state(new_state);
    //    }
    //}
}
