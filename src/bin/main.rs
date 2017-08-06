extern crate conway;
use conway::World;
use std::{thread, time};

fn main() {
    let mut world = World::new(10, 10);

    world.set_rules(|x| { 3; () });

    let run = true;
    while run {
        world.step();
        thread::sleep(time::Duration::from_secs(1));
    }
}
