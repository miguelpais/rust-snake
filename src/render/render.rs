use std::io;
use std::io::Write;

use std::thread::sleep;
use std::time::Duration;
use std::sync::mpsc::Receiver;

use crossterm::{
    cursor::MoveTo,
    style::{Color, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType},
    QueueableCommand,
};

use crate::util::even_ceiling;
use crate::domain::direction::Direction;
use crate::domain::command::Command;
use crate::domain::snake::Snake;

use super::fence;

const ONE_SECOND_MILIS: u64 = 1_000;


pub struct Render {
    snake: Snake,
    screen_width: u16,
    screen_height: u16,
    current_direction: Direction,
    frames_per_second: u64,
    input_receiver: Receiver<Command>
}

impl Render {
    pub fn new(size: u16, initial_snake_length: u8, frames_per_second: u64, input_receiver: Receiver<Command>) -> Render {
        let even_size = even_ceiling(size);
        let width = even_size * 2;
        let height = even_size;
        let half_screen = (even_size / 2) as u8;

        Render {
            screen_width: width,
            screen_height: height,
            snake: Snake::new(half_screen, half_screen, width, height, initial_snake_length),
            current_direction: Direction::LEFT,
            frames_per_second,
            input_receiver
        }
    }

    pub fn main_loop(&mut self) {
        self.init_display();
        fence::draw(0, self.screen_height, 0, self.screen_width);

        loop {
            let new_command = self.input_receiver.try_recv();
            match new_command {
                Ok(command) => {
                    match command {
                        Command::UP => if self.current_direction != Direction::DOWN { self.current_direction = Direction::UP },
                        Command::DOWN => if self.current_direction != Direction::UP { self.current_direction = Direction::DOWN },
                        Command::LEFT => if self.current_direction != Direction::RIGHT { self.current_direction = Direction::LEFT },
                        Command::RIGHT => if self.current_direction != Direction::LEFT { self.current_direction = Direction::RIGHT },
                        Command::EXIT => break
                    }
                },
                _ => ()
            }

            self.update_snake();
            self.draw_snake();

            sleep(Duration::from_millis(ONE_SECOND_MILIS / self.frames_per_second));
        }
    }

    pub fn init_display(&self) {
        let mut stdout = io::stdout();
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
        stdout.queue(SetForegroundColor(Color::DarkYellow)).unwrap();
    }

    fn update_snake(&mut self) {
        let mut stdout = io::stdout();
        let tail_point = self.snake.pos.last().unwrap();
        stdout.queue(MoveTo(tail_point.x, tail_point.y)).unwrap();
        print!(" ");
        self.snake.update_position(&self.current_direction);
    }

    fn draw_snake(&self) {
        let mut stdout = io::stdout();

        for pos in &self.snake.pos {
            stdout.queue(MoveTo(pos.x, pos.y)).unwrap();
            print!("{}", pos.direction.to_string());
        }
        stdout.queue(MoveTo(self.screen_width +1, self.screen_height + 1)).unwrap();
        stdout.flush().unwrap();
    }
}
