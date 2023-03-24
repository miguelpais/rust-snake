use std::sync::{Arc, mpsc, Mutex};
use std::thread;

mod snake;
use crate::snake::{screen::Screen, input};

use crossterm::terminal::{disable_raw_mode};

fn main() {
    let counter = Arc::new(Mutex::new(0));

    let lock_input_loop = Arc::clone(&counter);
    let lock_main_loop = Arc::clone(&counter);
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        input::input_loop(tx, lock_input_loop, 3);
    });

    let mut screen = Screen::new(40, 80);

    screen.main_loop(20, 5, rx, lock_main_loop);
    disable_raw_mode();
}
