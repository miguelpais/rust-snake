use std::sync::mpsc::Sender;
use std::thread::sleep;
use std::time::Duration;
use rusty_audio::Audio;

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::event::{read, Event, KeyCode, poll};

use crate::snake::command::Command;

pub fn input_loop(tx: Sender<Command>, input_capturing_window_ms: u64) {
    let mut audio = Audio::new();
    audio.add("audio_drop", "audio/audio_drop.wav");
    loop {
        {
            if let Some(command) = capture_command() {
                audio.play("audio_drop");
                tx.send(command).unwrap();
            }
        }
        sleep(Duration::from_millis(input_capturing_window_ms));
    }
}

fn capture_command() -> Option<Command> {
    let mut new_command = None;
    enable_raw_mode().unwrap_or(());

    let status = poll(Duration::from_millis(10));
    if status.is_ok() && status.unwrap() {
        let event = read();
        if event.is_ok() {
            let specific = event.unwrap();

            if specific == Event::Key(KeyCode::Right.into()) { new_command = Some(Command::RIGHT) }
            else if specific == Event::Key(KeyCode::Left.into()) { new_command = Some(Command::LEFT) }
            else if specific == Event::Key(KeyCode::Down.into()) { new_command = Some(Command::DOWN) }
            else if specific == Event::Key(KeyCode::Up.into()) { new_command = Some(Command::UP) }
            else if specific == Event::Key(KeyCode::Esc.into()) { new_command = Some(Command::EXIT) }
            else if specific == Event::Key(KeyCode::Char(' ').into()) { new_command = Some(Command::PAUSE) }
        }
    }

    disable_raw_mode().unwrap_or(());
    new_command
}
