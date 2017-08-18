use ncurses::{initscr, start_color, noecho, getmaxyx, stdscr, endwin};
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
    options: RenderOptions,
}
impl Renderer {
    pub fn new() -> Renderer {
        Self {
            options: RenderOptions::new(),
        }
    }

    pub fn initialize(&mut self) {
        initscr();
        start_color();
        noecho();
    }
    pub fn return_term_size(&self) -> (usize, usize) {
        let mut max_x = 0;
        let mut max_y = 0;
        getmaxyx(stdscr(), &mut max_y, &mut max_x);
        (max_x as usize / 2, max_y as usize)
    }
    
    pub fn end(&self) {
        endwin();
    }
    fn init_options(&mut self) {
        //set live/dead terminal colors
        let (live_bg, dead_bg) = if self.options.filled {
            (self.options.color.clone(), self.options.dead_color.clone())
        } else {
            (Color::Black, Color::Black)
        };
        set_color_pair(1, self.options.dead_color.clone(), dead_bg);
        set_color_pair(2, self.options.color.clone(), live_bg);
    }

    pub fn set_options(&mut self, options: RenderOptions) {
        self.options = options;
        self.init_options();
    }
    pub fn render(&self, grid: &Vec<Cell>) {
        erase();
        let mut buf: [u8; 4] = [0; 4];
        for (index, cell) in grid.iter().enumerate() {
            if let &CellState::Dead = cell.get_state() {
                if self.options.inverse {
                    attron(COLOR_PAIR(2));
                    &self.options.live_char.encode_utf8(&mut buf);
                    let s = str::from_utf8(&buf[..self.options.live_char.len_utf8()]).unwrap();
                    printw(s);
                } else {
                    attron(COLOR_PAIR(1));
                    &self.options.dead_char.encode_utf8(&mut buf);
                    let s = str::from_utf8(&buf[..self.options.dead_char.len_utf8()]).unwrap();
                    printw(s);
                }
            } else {
                if self.options.inverse {
                    attron(COLOR_PAIR(1));
                    &self.options.dead_char.encode_utf8(&mut buf);
                    let s = str::from_utf8(&buf[..self.options.dead_char.len_utf8()]).unwrap();
                    printw(s);
                } else {
                    attron(COLOR_PAIR(2));
                    &self.options.live_char.encode_utf8(&mut buf);
                    let s = str::from_utf8(&buf[..self.options.live_char.len_utf8()]).unwrap();
                    printw(s);
                }
            }
            if index % self.options.width == self.options.width-1 {
                printw("\n");
            } else {
                if self.options.padding {
                    printw(" ");
                }
            }
        }
        // -z
        if self.options.time_slice { println!(" "); }
        refresh();
        thread::sleep(time::Duration::from_millis(self.options.delay));
    }
}
