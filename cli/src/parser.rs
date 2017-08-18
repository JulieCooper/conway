extern crate argparse;
use self::argparse::{ArgumentParser, StoreTrue, StoreOption, Store};
use config::Config;

pub struct Parser {
    config: Config,
}
impl Parser {
    pub fn new(config: Config) -> Self {
        Parser {
            config: config,
        }
    }
    pub fn parse(&mut self) -> Config {
        {
            let mut ap = ArgumentParser::new();
            //live_char
            ap.refer(&mut self.config.live_char)
                .add_option(&["-l", "--live-char"], Store,
                            "Alive character");
            //dead_char
            ap.refer(&mut self.config.dead_char)
                .add_option(&["-d", "--dead-char"], Store,
                            "Dead character");
            //output_file
            ap.refer(&mut self.config.output_file)
                .add_option(&["-o", "--output-file"], Store,
                            "File for debug output");
            //initial_state
            ap.refer(&mut self.config.initial_state)
                .add_option(&["-i", "--init"], Store,
                            "Initial grid state");
            //adjacent_rules
            ap.refer(&mut self.config.adjacent_rules)
                .add_option(&["-a", "--adjacent-rules"], Store,
                            "Input cell pattern");
            //ruleset
            ap.refer(&mut self.config.ruleset)
                .add_option(&["-r", "--ruleset"], Store,
                            "Ruleset");
            //delay
            ap.refer(&mut self.config.delay)
                .add_option(&["-d", "--delay"], Store,
                            "Delay between ticks");
            //width
            ap.refer(&mut self.config.width)
                .add_option(&["-w", "--width"], Store,
                            "Width of grid in cells");
            //height
            ap.refer(&mut self.config.height)
                .add_option(&["-h", "--height"], Store,
                            "Height of grid in cells");
            //filled
            ap.refer(&mut self.config.filled)
                .add_option(&["-f", "--filled"], StoreTrue,
                            "Fill cells instead of printing character");
            //inverse
            ap.refer(&mut self.config.inverse)
                .add_option(&["-i", "--inverse"], StoreTrue,
                            "Inverse live and dead cell display");
            //time_slice
            ap.refer(&mut self.config.time_slice)
                .add_option(&["-z", "--time-slice"], StoreTrue,
                            "Show progress of grid in time");
            //color FIXME: change to live_color
            ap.refer(&mut self.config.color)
                .add_option(&["-c", "--color"], Store,
                            "Color for live cells");
            //dead_color
            ap.refer(&mut self.config.dead_color)
                .add_option(&["--dead-color"], Store,
                            "Color for dead cells");
            ap.parse_args();
        }
        self.config.clone()
    }
}
