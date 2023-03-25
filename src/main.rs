use std::sync::mpsc;
use std::thread;

use crossterm::terminal::{disable_raw_mode};

mod input;
mod domain;
mod render;
mod util;

use crate::render::render::Render;
use input::input::input_loop;

const SCREEN_SIZE:u16 = 40;
const INITIAL_SNAKE_LENGTH: u8 = 10;
const FRAMES_PER_SECOND: u64 = 8;
const INPUT_CAPTURING_WINDOW_MS: u64 = 3;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        input_loop(tx, INPUT_CAPTURING_WINDOW_MS);
    });

    let mut render = Render::new(
        SCREEN_SIZE,
        INITIAL_SNAKE_LENGTH,
        FRAMES_PER_SECOND,
        rx);

    render.main_loop();

    disable_raw_mode().unwrap_or(());
}
