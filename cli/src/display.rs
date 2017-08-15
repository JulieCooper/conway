use ncurses::printw;
use conway::world::RenderParameters;
use conway::world::cell::{Cell, CellState};
use std::{thread, time};

pub struct Renderer {
    delay: u64,
    alive_char: char,
    dead_char: char,
    width: usize, height: u64,
}
impl Renderer {
    pub fn new(params: RenderParameters, width: usize) -> Renderer {
        Self {
            delay: params.delay,
            alive_char: params.alive_char,
            dead_char: params.dead_char,
            width: width,
            height: params.height,
        }
    }
    pub fn render(&self, grid: &Vec<Cell>) {
        let mut row = String::new();
        for (index, cell) in grid.iter().enumerate() {
            if let &CellState::Dead = cell.get_state() {
                row.push(' ');
            } else {
                row.push('o');
            }

            if index % self.width == self.width - 1 {
                row.push('\n');
                printw(&row.to_string());
                row.clear();
            } else {
                row.push(' ');
            }
        }
        thread::sleep(time::Duration::from_millis(self.delay));
    }
}
