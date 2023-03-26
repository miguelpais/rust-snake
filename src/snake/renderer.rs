use std::thread::sleep;
use std::time::Duration;
use std::sync::mpsc::Receiver;

use crate::util::even_ceiling;
use crate::snake::command::Command;
use crate::snake::snake::Snake;
use crate::snake::display;

const ONE_SECOND_MILIS: u64 = 1_000;

pub struct Renderer {
    snake: Snake,
    screen_width: u16,
    screen_height: u16,
    frame_duration: Duration,
    input_receiver: Receiver<Command>
}

impl Renderer {
    pub fn new(screen_size: u16, initial_snake_length: u8, frames_per_second: u64, input_receiver: Receiver<Command>) -> Renderer {
        let even_screen_size = even_ceiling(screen_size);
        let screen_width = even_screen_size * 2;
        let screen_height = even_screen_size;
        let half_screen = (even_screen_size / 2) as u8;
        let frame_duration = Duration::from_millis(ONE_SECOND_MILIS / frames_per_second);

        let snake = Snake::new(half_screen, half_screen, screen_width, screen_height, initial_snake_length);

        Renderer {
            screen_width,
            screen_height,
            snake,
            frame_duration,
            input_receiver
        }
    }

    pub fn main_loop(&mut self) {
        display::init();
        display::draw_fence(0, self.screen_height, 0, self.screen_width);

        loop {
            let tail = self.snake.pos.last().unwrap();
            display::erase_at_position(tail.x, tail.y);

            let message = self.input_receiver.try_recv();
            if let Ok(Command::EXIT) = message { break }
            if let Ok(command) = message { self.snake.change_direction(command.to_direction()) }

            self.snake.proceed();
            display::draw_snake(&self.snake);

            sleep(self.frame_duration);
        }
    }
}
