use std::sync::mpsc;
use std::thread;

mod snake;
use crate::snake::{screen::Screen, input};
use snake::util::even_ceiling;

use crossterm::terminal::{disable_raw_mode};

const SCREEN_SIZE:u16 = 40;
const FRAMES_PER_SECOND: u64 = 8;
const INPUT_CAPTURING_WINDOW_MS: u64 = 3;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        input::input_loop(tx, INPUT_CAPTURING_WINDOW_MS);
    });

    let mut screen = Screen::new(even_ceiling(SCREEN_SIZE));

    screen.main_loop(FRAMES_PER_SECOND, rx);
    disable_raw_mode();
}
