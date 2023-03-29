use std::sync::mpsc;
use std::thread;

use crate::snake::event;
use crate::snake::renderer::Renderer;

use crossterm::terminal::disable_raw_mode;

pub fn start(screen_size: u16, initial_snake_length: u8,
             frames_per_second: u64, floating_walls_mode: bool) {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        event::event_loop(tx);
    });

    let mut renderer = Renderer::new(
        screen_size,
        initial_snake_length,
        frames_per_second,
        floating_walls_mode,
        rx);

    renderer.main_loop();

    disable_raw_mode().unwrap_or(());
}
