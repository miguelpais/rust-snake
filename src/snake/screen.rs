use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};

use ascii_canvas::{{AsciiCanvas, AsciiView}};

use super::direction::Direction;
use super::command::Command;
use super::body::SnakeBody;

use ascii_canvas::style::DEFAULT;

use std::thread::sleep;
use std::time::Duration;

pub struct Screen {
    canvas: AsciiCanvas,
    snake: SnakeBody,
    width: usize,
    height: usize,
    direction: Direction,
}

impl Screen {
    pub fn new(height: usize, width: usize) -> Screen {
        let mut canvas = AsciiCanvas::new(height, width);
        {
            let view: &mut dyn AsciiView = &mut canvas;
            view.draw_vertical_line(0..height, 0);
            view.draw_vertical_line(0..height, width - 1);
            view.draw_horizontal_line(0, 0..width);
            view.draw_horizontal_line(height - 1, 0..width);
        }
        Screen {
            canvas,
            width,
            height,
            snake: SnakeBody::new(height / 2, width / 2, height, width),
            direction: Direction::LEFT,
        }
    }

    pub fn main_loop(&mut self, frames_per_second: u64, rx: Receiver<Command>, lock: Arc<Mutex<i32>>) {
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

            self.update();
            self.draw_screen(frame_ttl_ms, &lock);
        }
    }

    fn update(&mut self) {
        self.canvas.write_char(self.snake.pos[self.snake.length-1].row, self.snake.pos[self.snake.length-1].column, ' ', DEFAULT);
        self.snake.update_position(&self.direction);

        for idx in 0..self.snake.length {
            self.canvas.write_char(self.snake.pos[idx].row, self.snake.pos[idx].column, self.snake.pos[idx].direction.to_string(), DEFAULT);
        }

    }

    fn draw_screen(&self, frame_ttl_ms: u64, lock: &Arc<Mutex<i32>>) {
        sleep(Duration::from_millis(frame_ttl_ms));
        let mut num = lock.lock().unwrap();
        print!("{}[2J", 27 as char);
        for row in self.canvas.to_strings() {
            println!("{}", row);
        }
        *num = 1;
    }
}
