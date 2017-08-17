use ncurses::{erase, refresh, printw, init_pair, attron, COLOR_PAIR};
use ncurses::{COLOR_BLACK, COLOR_RED, COLOR_GREEN, COLOR_YELLOW, COLOR_BLUE, COLOR_MAGENTA, COLOR_CYAN, COLOR_WHITE};
use conway::world::cell::{Cell, CellState};
use config::{RenderOptions, Color};
use std::{thread, time, str};

pub struct Renderer {
    delay: u64,
    live_char: char,
    dead_char: char,
    width: usize,
    //height: u64,
    //?
    time_slice: bool,
}
impl Renderer {
    pub fn new(params: RenderOptions, width: usize) -> Renderer {
        if params.filled {
            match params.color {
                Color::Black => init_pair(2, COLOR_BLACK, COLOR_BLACK),
                Color::Red => init_pair(2, COLOR_RED, COLOR_RED),
                Color::Green => init_pair(2, COLOR_GREEN, COLOR_GREEN),
                Color::Yellow => init_pair(2, COLOR_YELLOW, COLOR_YELLOW),
                Color::Blue => init_pair(2, COLOR_BLUE, COLOR_BLUE),
                Color::Magenta => init_pair(2, COLOR_MAGENTA, COLOR_MAGENTA),
                Color::Cyan => init_pair(2, COLOR_CYAN, COLOR_CYAN),
                Color::White => init_pair(2, COLOR_WHITE, COLOR_WHITE),
            };
        } else {
            match params.color {
                Color::Black => init_pair(2, COLOR_BLACK, COLOR_BLACK),
                Color::Red => init_pair(2, COLOR_RED, COLOR_BLACK),
                Color::Green => init_pair(2, COLOR_GREEN, COLOR_BLACK),
                Color::Yellow => init_pair(2, COLOR_YELLOW, COLOR_BLACK),
                Color::Blue => init_pair(2, COLOR_BLUE, COLOR_BLACK),
                Color::Magenta => init_pair(2, COLOR_MAGENTA, COLOR_BLACK),
                Color::Cyan => init_pair(2, COLOR_CYAN, COLOR_BLACK),
                Color::White => init_pair(2, COLOR_WHITE, COLOR_BLACK),
            };
        }
        Self {
            delay: params.delay,
            live_char: params.live_char,
            dead_char: params.dead_char,
            width: width,
            //height: params.height,
            time_slice: params.time_slice,
        }
    }
    pub fn render(&self, grid: &Vec<Cell>) {
        erase();
        let mut buf: [u8; 4] = [0; 4];
        for (index, cell) in grid.iter().enumerate() {
            if let &CellState::Dead = cell.get_state() {
                &self.dead_char.encode_utf8(&mut buf);
                let s = str::from_utf8(&buf[..self.dead_char.len_utf8()]).unwrap();

                attron(COLOR_PAIR(1));
                printw(s);
                //printw(&self.dead_char.to_string());
            } else {
                &self.live_char.encode_utf8(&mut buf);
                let s = str::from_utf8(&buf[..self.live_char.len_utf8()]).unwrap();

                attron(COLOR_PAIR(2));
                printw(s);
                //printw(&self.live_char.to_string());
            }

            if index % self.width == self.width - 1 {
                printw("\n");
            } else {
                printw(" ");
            }
        }
        if self.time_slice { println!(" "); }
        refresh();
        thread::sleep(time::Duration::from_millis(self.delay));
    }
}
