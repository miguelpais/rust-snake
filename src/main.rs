use std::sync::{Arc, mpsc, Mutex};
use std::thread;

mod snake;
use crate::snake::{screen::Screen, input};

use crossterm::terminal::{disable_raw_mode};

const SCREEN_HEIGTH:usize = 40;
const SCREEN_WIDTH:usize = 80;
const FRAMES_PER_SECOND: u64 = 10;
const INPUT_CAPTURING_WINDOW_MS: u64 = 3;

fn main() {
    let counter = Arc::new(Mutex::new(0));

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        input::input_loop(tx, INPUT_CAPTURING_WINDOW_MS);
    });

    let mut screen = Screen::new(SCREEN_HEIGTH, SCREEN_WIDTH);

    screen.main_loop(FRAMES_PER_SECOND, rx);
    disable_raw_mode();
}
