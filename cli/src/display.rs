use ncurses::{clear, erase, refresh, printw, init_pair, attron, COLOR_PAIR};
use ncurses::{COLOR_BLACK, COLOR_RED, COLOR_GREEN, COLOR_BLUE, COLOR_WHITE};
use conway::world::cell::{Cell, CellState};
use config::{Render_Options, Color};
use std::{thread, time};

pub struct Renderer {
    delay: u64,
    live_char: char,
    dead_char: char,
    width: usize,
    height: u64,
}
impl Renderer {
    pub fn new(params: Render_Options, width: usize) -> Renderer {
        if params.filled {
            match params.color {
                Color::Red => init_pair(2, COLOR_RED, COLOR_RED),
                Color::Green => init_pair(2, COLOR_GREEN, COLOR_GREEN),
                Color::Blue => init_pair(2, COLOR_BLUE, COLOR_BLUE),
                Color::White => init_pair(2, COLOR_WHITE, COLOR_WHITE),
            };
        } else {
            match params.color {
                Color::Red => init_pair(2, COLOR_RED, COLOR_BLACK),
                Color::Green => init_pair(2, COLOR_GREEN, COLOR_BLACK),
                Color::Blue => init_pair(2, COLOR_BLUE, COLOR_BLACK),
                Color::White => init_pair(2, COLOR_WHITE, COLOR_BLACK),
            };
        }
        Self {
            delay: params.delay,
            live_char: params.live_char,
            dead_char: params.dead_char,
            width: width,
            height: params.height,
        }
    }
    pub fn render(&self, grid: &Vec<Cell>) {
        erase();
        for (index, cell) in grid.iter().enumerate() {
            if let &CellState::Dead = cell.get_state() {
                attron(COLOR_PAIR(1));
                printw(&self.dead_char.to_string());
            } else {
                attron(COLOR_PAIR(2));
                printw(&self.live_char.to_string());
            }

            if index % self.width == self.width - 1 {
                printw("\n");
            } else {
                printw(" ");
            }
        }
        refresh();
        thread::sleep(time::Duration::from_millis(self.delay));
    }
}
