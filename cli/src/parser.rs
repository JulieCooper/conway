extern crate argparse;
use self::argparse::{ArgumentParser, StoreTrue, Store};
use Config;

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
            ////output_file
            //ap.refer(&mut self.config.world_options.output_file)
            //    .add_option(&["-o", "--output-file"], Store,
            //                "File for debug output");
            //initial_state
            ap.refer(&mut self.config.world_options.init)
                .add_option(&["-i", "--initial-state"], Store,
                            "Initial grid state");
            //adjacent_rules
            ap.refer(&mut self.config.world_options.input_cells)
                .add_option(&["-a", "--input-cells"], Store,
                            "Input cell pattern");
            //rule
            ap.refer(&mut self.config.world_options.rules)
                .add_option(&["-r", "--ruleset"], Store,
                            "Ruleset");
            //delay
            ap.refer(&mut self.config.render_options.delay)
                .add_option(&["-d", "--delay"], Store,
                            "Delay between ticks");
            //width
            ap.refer(&mut self.config.render_options.width)
                .add_option(&["-w", "--width"], Store,
                            "Width of grid in cells");
            //height
            ap.refer(&mut self.config.render_options.height)
                .add_option(&["-h", "--height"], Store,
                            "Height of grid in cells");
            //live_char
            ap.refer(&mut self.config.render_options.live_char)
                .add_option(&["-l", "--live-char"], Store,
                            "Alive character");
            //dead_char
            ap.refer(&mut self.config.render_options.dead_char)
                .add_option(&["-d", "--dead-char"], Store,
                            "Dead character");
            //filled
            ap.refer(&mut self.config.render_options.filled)
                .add_option(&["-f", "--filled"], StoreTrue,
                            "Fill cells instead of printing character");
            //inverse
            ap.refer(&mut self.config.render_options.inverse)
                .add_option(&["-i", "--inverse"], StoreTrue,
                            "Inverse live and dead cell display");
            //time_slice
            ap.refer(&mut self.config.render_options.time_slice)
                .add_option(&["-z", "--time-slice"], StoreTrue,
                            "Show progress of grid in time");
            //color FIXME: change to live_color
            ap.refer(&mut self.config.render_options.color)
                .add_option(&["-c", "--color"], Store,
                            "Color for live cells");
            //dead_color
            ap.refer(&mut self.config.render_options.dead_color)
                .add_option(&["--dead-color"], Store,
                            "Color for dead cells");
            match ap.parse_args() {
                Ok(_) => (),
                Err(_e) => (),
            }
        }
        self.config.clone()
    }
}
