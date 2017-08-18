use ncurses::{erase, refresh, printw, init_pair, attron, COLOR_PAIR};
use ncurses::{COLOR_BLACK, COLOR_RED, COLOR_GREEN, COLOR_YELLOW, COLOR_BLUE, COLOR_MAGENTA, COLOR_CYAN, COLOR_WHITE};
use conway::world::cell::{Cell, CellState};
use config::{RenderOptions, Color};
use std::{thread, time, str};

fn color_to_i16(color: Color) -> i16 {
    match color {
        Color::Black => COLOR_BLACK,
        Color::Red => COLOR_RED,
        Color::Green => COLOR_GREEN,
        Color::Yellow => COLOR_YELLOW,
        Color::Blue => COLOR_BLUE,
        Color::Magenta => COLOR_MAGENTA,
        Color::Cyan => COLOR_CYAN,
        Color::White => COLOR_WHITE,
    }
}
fn set_color_pair(id: i16, bg_color: Color, fg_color: Color) {
    init_pair(id,
              color_to_i16(bg_color),
              color_to_i16(fg_color));
}
pub struct Renderer {
    delay: u64,
    live_char: char,
    dead_char: char,
    width: usize,
    //height: u64,
    inverse: bool,
    //?
    time_slice: bool,
}
impl Renderer {
    pub fn new(params: RenderOptions, width: usize) -> Renderer {
        let (live_bg, dead_bg) = if params.filled {
            (params.color.clone(), params.dead_color.clone())
        } else {
            (Color::Black, Color::Black)
        };
        //set_color_pair(1, dead_bg, params.dead_color);
        //set_color_pair(2, live_bg, params.color);
        set_color_pair(1, params.dead_color, dead_bg);
        set_color_pair(2, params.color, live_bg);

        Self {
            delay: params.delay,
            live_char: params.live_char,
            dead_char: params.dead_char,
            width: width,
            //height: params.height,
            inverse: params.inverse,
            time_slice: params.time_slice,
        }
    }
    pub fn render(&self, grid: &Vec<Cell>) {
        erase();
        let mut buf: [u8; 4] = [0; 4];
        for (index, cell) in grid.iter().enumerate() {
            if let &CellState::Dead = cell.get_state() {
                if self.inverse {
                    attron(COLOR_PAIR(2));
                    &self.live_char.encode_utf8(&mut buf);
                    let s = str::from_utf8(&buf[..self.live_char.len_utf8()]).unwrap();
                    printw(s);
                } else {
                    attron(COLOR_PAIR(1));
                    &self.dead_char.encode_utf8(&mut buf);
                    let s = str::from_utf8(&buf[..self.dead_char.len_utf8()]).unwrap();
                    printw(s);
                }
            } else {
                if self.inverse {
                    attron(COLOR_PAIR(1));
                    &self.dead_char.encode_utf8(&mut buf);
                    let s = str::from_utf8(&buf[..self.dead_char.len_utf8()]).unwrap();
                    printw(s);
                } else {
                    attron(COLOR_PAIR(2));
                    &self.live_char.encode_utf8(&mut buf);
                    let s = str::from_utf8(&buf[..self.live_char.len_utf8()]).unwrap();
                    printw(s);
                }
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
