extern crate lifegame;

use std::{thread, time};
use lifegame::Game;

fn main() {
    let interval = time::Duration::from_secs(1);
    let mut game = Game::new(30, 90);
    loop {
        game.update();
        game.render();
        thread::sleep(interval);
    }
}
