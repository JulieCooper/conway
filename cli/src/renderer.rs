use ncurses::{initscr, start_color, noecho, getmaxyx, stdscr, endwin};
use ncurses::{erase, refresh, printw, init_pair, attron, COLOR_PAIR};
use ncurses::{raw, nodelay};
use ncurses::{COLOR_BLACK, COLOR_RED, COLOR_GREEN, COLOR_YELLOW, COLOR_BLUE, COLOR_MAGENTA, COLOR_CYAN, COLOR_WHITE};
use conway::world::cell::{Cell, CellState};
use std::{thread, time, str};
use std::str::FromStr;

#[derive(Clone)]
pub struct RenderOptions {
    pub width: usize,
    pub height: usize,
    pub delay: u64,
    pub live_char: String,
    pub dead_char: String,
    pub filled: bool,
    pub inverse: bool,
    pub padding: bool,
    pub stopped: bool,
    pub color: Color,
    pub dead_color: Color,
    //?
    pub time_slice: bool,
}
impl RenderOptions {
    pub fn new() -> Self {
        RenderOptions {
            width: 0,
            height: 0,
            delay: 0,
            live_char: String::from(" "),
            dead_char: String::from(" "),
            filled: false,
            inverse: false,
            padding: false,
            stopped: false,
            color: Color::White,
            dead_color: Color::White,
            time_slice: false,
        }
    }
}

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
        raw();
        nodelay(stdscr(), true);
        start_color();
        noecho();
    }

    pub fn end(&self) {
        endwin();
    }

    pub fn return_term_size(&self) -> (usize, usize) {
        let mut max_x = 0;
        let mut max_y = 0;
        getmaxyx(stdscr(), &mut max_y, &mut max_x);
        (max_x as usize / 2, max_y as usize)
    }

    pub fn set_options(&mut self, options: RenderOptions) {
        self.options = options;
        self.init_options();
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

    pub fn render(&self, grid: &Vec<Cell>) {
        erase();
        let mut buf: [u8; 4] = [0; 4];
        for (index, cell) in grid.iter().enumerate() {
            if let &CellState::Dead = cell.get_state() {
                if self.options.inverse {
                    attron(COLOR_PAIR(2));
                    //&self.options.live_char.encode_utf8(&mut buf);
                    //let s = str::from_utf8(&buf[..self.options.live_char.len_utf8()]).unwrap();
                    printw(&self.options.live_char);
                } else {
                    attron(COLOR_PAIR(1));
                    //&self.options.dead_char.encode_utf8(&mut buf);
                    //let s = str::from_utf8(&buf[..self.options.dead_char.len_utf8()]).unwrap();
                    printw(&self.options.dead_char);
                }
            } else {
                if self.options.inverse {
                    attron(COLOR_PAIR(1));
                    //&self.options.dead_char.encode_utf8(&mut buf);
                    //let s = str::from_utf8(&buf[..self.options.dead_char.len_utf8()]).unwrap();
                    printw(&self.options.dead_char);
                } else {
                    attron(COLOR_PAIR(2));
                    //&self.options.live_char.encode_utf8(&mut buf);
                    //let s = str::from_utf8(&buf[..self.options.live_char.len_utf8()]).unwrap();
                    printw(&self.options.live_char);
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

#[derive(Clone)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}
impl FromStr for Color {
    type Err = ();
    fn from_str(s: &str) -> Result<Color, ()> {
        match s {
            "Black" => Ok(Color::Black),
            "Red" => Ok(Color::Red),
            "Green" => Ok(Color::Green),
            "Yellow" => Ok(Color::Yellow),
            "Blue" => Ok(Color::Blue),
            "Magenta" => Ok(Color::Magenta),
            "Cyan" => Ok(Color::Cyan),
            "White" => Ok(Color::White),
            _ => Err(()),
        }
    }
}
