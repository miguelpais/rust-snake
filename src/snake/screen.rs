use std::sync::mpsc::Receiver;

use super::direction::Direction;
use super::command::Command;
use super::body::SnakeBody;
use super::util::even_ceiling;

use crossterm::{
    cursor::MoveTo,
    style::{Color, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType},
    QueueableCommand,
};
use std::io;
use std::io::Write;

use std::thread::sleep;
use std::time::Duration;

pub struct Screen {
    snake: SnakeBody,
    width: u16,
    height: u16,
    direction: Direction,
}

impl Screen {
    fn draw_square(y_max: u16, x_max: u16) {
        let mut stdout = io::stdout();

        stdout.queue(MoveTo(0, 0)).unwrap();
        {print!("{}", "┌") }
        stdout.queue(MoveTo(x_max, 0)).unwrap();
        {print!("{}", "┐") }
        stdout.queue(MoveTo(0, y_max)).unwrap();
        {print!("{}", "└") }
        stdout.queue(MoveTo(x_max, y_max)).unwrap();
        {print!("{}", "┘") }

        for i in 1..x_max {
            stdout.queue(MoveTo(i, 0)).unwrap();
            {print!("{}", "─") }
            stdout.queue(MoveTo(i, y_max)).unwrap();
            {print!("{}", "─") }
        }

        for i in 1..y_max {
            stdout.queue(MoveTo(0, i)).unwrap();
            { print!("{}", "│") }
            stdout.queue(MoveTo(x_max, i)).unwrap();
            { print!("{}", "│") }
        }
    }

    pub fn draw_canvas(&self) {
        let mut stdout = io::stdout();
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
        stdout.queue(SetForegroundColor(Color::DarkYellow)).unwrap();

        Screen::draw_square(self.height, self.width);
    }
    pub fn new(size: u16) -> Screen {
        Screen {
            width: size*2,
            height: size,
            snake: SnakeBody::new(size / 2, size / 2, size, size*2),
            direction: Direction::LEFT,
        }
    }

    pub fn main_loop(&mut self, frames_per_second: u64, rx: Receiver<Command>) {
        self.draw_canvas();
        let frame_ttl_ms = 1000 / frames_per_second;

        loop {
            let new_command = rx.try_recv();
            match new_command {
                Ok(command) => {
                    match command {
                        Command::UP => if self.direction != Direction::DOWN { self.direction = Direction::UP },
                        Command::DOWN => if self.direction != Direction::UP { self.direction = Direction::DOWN },
                        Command::LEFT => if self.direction != Direction::RIGHT { self.direction = Direction::LEFT },
                        Command::RIGHT => if self.direction != Direction::LEFT { self.direction = Direction::RIGHT },
                        Command::NONE => (),
                        Command::EXIT => break
                    }
                },
                _ => ()
            }

            self.update_snake();
            self.draw_snake(frame_ttl_ms);
        }
    }

    fn update_snake(&mut self) {
        let mut stdout = io::stdout();
        let tail_point = self.snake.pos.last().unwrap();
        stdout.queue(MoveTo(tail_point.column, tail_point.row)).unwrap();
        print!(" ");
        self.snake.update_position(&self.direction);
    }

    fn draw_snake(&self, frame_ttl_ms: u64) {
        sleep(Duration::from_millis(frame_ttl_ms));
        let mut stdout = io::stdout();

        for pos in &self.snake.pos {
            stdout.queue(MoveTo(pos.column, pos.row)).unwrap();
            print!("{}", pos.direction.to_string());
        }
        stdout.queue(MoveTo(self.width+1, self.height + 1)).unwrap();
        stdout.flush().unwrap();
    }
}
