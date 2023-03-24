use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;
use super::command::Command;
use std::thread::sleep;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
extern crate crossterm;

use crossterm::event::{read, Event, KeyCode, poll};
use std::time::Duration;

pub fn input_loop(tx: Sender<Command>, lock: Arc<Mutex<i32>>, input_capturing_window_ms: u64) {
    loop {
        {
            let mut num = lock.lock().unwrap();
            *num = 1;
            match capture_command() {
                Some(new_command) => tx.send(new_command).unwrap(),
                _ => ()
            }
        }
        sleep(Duration::from_millis(input_capturing_window_ms));
    }
}

fn capture_command() -> Option<Command> {
    let mut new_command = None;
    enable_raw_mode();
    let status = poll(Duration::from_millis(10));
    if status.is_ok() && status.unwrap() {
        let event = read();
        if event.is_ok() {
            let specific = event.unwrap();
            if specific == Event::Key(KeyCode::Right.into()) {
                new_command = Some(Command::RIGHT)
            }
            else if specific == Event::Key(KeyCode::Left.into()) {
                new_command = Some(Command::LEFT)
            }
            else if specific == Event::Key(KeyCode::Down.into()) {
                new_command = Some(Command::DOWN)
            }
            else if specific == Event::Key(KeyCode::Up.into()) {
                new_command = Some(Command::UP)
            }
            else if specific == Event::Key(KeyCode::Esc.into()) {
                new_command = Some(Command::EXIT)
            }
        }
    }
    disable_raw_mode();
    new_command
}
