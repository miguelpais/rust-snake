use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};

use ascii_canvas::{{AsciiCanvas, AsciiView}};

use super::direction::Direction;
use super::command::Command;

use ascii_canvas::style::DEFAULT;

use std::thread::sleep;
use std::time::Duration;

#[derive(Clone)]
struct Point {
    row: usize,
    column: usize,
    direction: Direction
}

struct SnakeBody {
    pos: Vec<Point>,
    length: usize,
}

const SNAKE_LENGTH: usize = 10;

impl SnakeBody {
    pub fn new(row_head: usize, column_head: usize) -> SnakeBody {

        SnakeBody {
            pos: (0..SNAKE_LENGTH).map(|el| Point {
                row: row_head,
                column: column_head + el * 2,
                direction: Direction::LEFT
            }).collect(),
            length: SNAKE_LENGTH
        }
    }

    pub fn update_position(&mut self, direction: &Direction, screen_width: usize, screen_height: usize) {
        let head_pos = &mut self.pos[0];
        let mut previous_body_part_pos = Point {
            row: head_pos.row,
            column: head_pos.column,
            direction: head_pos.direction.clone()
        };

        head_pos.direction = direction.clone();
        match direction {
            Direction::UP => head_pos.row = previous_body_part_pos.row - 1,
            Direction::DOWN => head_pos.row = previous_body_part_pos.row + 1,
            Direction::LEFT => head_pos.column = previous_body_part_pos.column - 2,
            Direction::RIGHT => head_pos.column = previous_body_part_pos.column + 2
        }
        if head_pos.row == screen_height - 1 {
            head_pos.row = 1;
        } else if head_pos.row == 0 {
            head_pos.row = screen_height - 2;
        }
        if head_pos.column >= screen_width - 1 {
            head_pos.column = 1
        } else if head_pos.column <= 1 {
            head_pos.column = screen_width - 2
        }

        for idx in 1..self.length {
            let new_previous_pos = Point {
                row: self.pos[idx].row,
                column: self.pos[idx].column,
                direction: self.pos[idx].direction.clone()
            };
            self.pos[idx].column = previous_body_part_pos.column;
            self.pos[idx].row = previous_body_part_pos.row;
            self.pos[idx].direction = previous_body_part_pos.direction;
            previous_body_part_pos = new_previous_pos;
        }
    }
}

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
            snake: SnakeBody::new(height / 2, width / 2),
            direction: Direction::LEFT,
        }
    }

    pub fn main_loop(&mut self, frames_per_second: u64, update_every_n_frames: u8, rx: Receiver<Command>, lock: Arc<Mutex<i32>>) {
        let frame_ttl_ms = 1000 / frames_per_second;
        let mut frame_counter = 0;

        loop {
            let new_command = rx.try_recv();
            match (new_command) {
                Ok(command) => {
                    match (command) {
                        Command::UP => {
                            if self.direction != Direction::DOWN {
                                self.direction = Direction::UP
                            }
                        },
                        Command::DOWN => {
                            if self.direction != Direction::UP {
                                self.direction = Direction::DOWN
                            }
                        }
                        Command::LEFT => {
                            if self.direction != Direction::RIGHT {
                                self.direction = Direction::LEFT
                            }
                        }
                        Command::RIGHT => {
                            {
                                if self.direction != Direction::LEFT {
                                    self.direction = Direction::RIGHT
                                }
                            }
                        },
                        Command::NONE => (),
                        Command::EXIT => break
                    }
                },
                _ => ()
            }
            if frame_counter > update_every_n_frames {
                frame_counter = 0;
                self.update();

            }
            frame_counter += 1;
            self.draw_screen(frame_ttl_ms, &lock);
        }
    }

    fn update(&mut self) {
        self.canvas.write_char(self.snake.pos[self.snake.length-1].row, self.snake.pos[self.snake.length-1].column, ' ', DEFAULT);
        self.snake.update_position(&self.direction, self.width, self.height);

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
