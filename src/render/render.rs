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
use crate::domain::point::Point;

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
    pub fn new(screen_size: u16, initial_snake_length: u8, frames_per_second: u64, input_receiver: Receiver<Command>) -> Render {
        let even_screen_size = even_ceiling(screen_size);
        let screen_width = even_screen_size * 2;
        let screen_height = even_screen_size;
        let half_screen = (even_screen_size / 2) as u8;

        let snake = Snake::new(half_screen, half_screen, screen_width, screen_height, initial_snake_length);

        Render {
            current_direction: Direction::LEFT,
            screen_width,
            screen_height,
            snake,
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

            let last_tail_point = self.update_snake();
            self.draw_snake(last_tail_point);

            sleep(Duration::from_millis(ONE_SECOND_MILIS / self.frames_per_second));
        }
    }

    fn update_snake(&mut self) -> Point {
        let tail = self.snake.pos.last().unwrap();

        let tail_point = Point {
            x: tail.x,
            y: tail.y,
            direction: Direction::NONE
        };

        self.snake.update_position(&self.current_direction);

        tail_point
    }

    fn draw_snake(&self, last_tail_point: Point) {
        let mut stdout = io::stdout();

        stdout.queue(MoveTo(last_tail_point.x, last_tail_point.y)).unwrap();
        print!(" ");

        for pos in &self.snake.pos {
            stdout.queue(MoveTo(pos.x, pos.y)).unwrap();
            print!("{}", pos.direction.to_string());
        }
        stdout.queue(MoveTo(self.screen_width +1, self.screen_height + 1)).unwrap();
        stdout.flush().unwrap();
    }

    pub fn init_display(&self) {
        let mut stdout = io::stdout();
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
        stdout.queue(SetForegroundColor(Color::DarkYellow)).unwrap();
    }
}
