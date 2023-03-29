use std::thread::sleep;
use std::time::Duration;
use std::sync::mpsc::Receiver;
use rusty_audio::Audio;

use crate::util::even_ceiling;
use crate::snake::command::Command;
use crate::snake::snake::Snake;
use crate::snake::display;
use crate::snake::food::Food;
use crate::snake::point::Point;
use crate::snake::renderer::ColisionEvent::{CollidedWithBody, CollidedWithFence};

const ONE_SECOND_MILLIS: u64 = 1_000;

pub struct Renderer {
    snake: Snake,
    food: Food,
    screen_width: u16,
    screen_height: u16,
    frame_duration: Duration,
    input_receiver: Receiver<Command>,
    captured_food: Vec<Point>,
    score: u16,
    game_over: bool,
    pause: bool,
    audio: Audio,
    floating_walls_mode: bool,
}

enum ColisionEvent {
    CollidedWithBody,
    CollidedWithFence
}

impl Renderer {
    pub fn new(screen_size: u16, initial_snake_length: u8, frames_per_second: u64,
               floating_walls_mode: bool, input_receiver: Receiver<Command>) -> Renderer {
        let even_screen_size = even_ceiling(screen_size);
        let screen_width = even_screen_size * 2;
        let screen_height = even_screen_size;
        let half_screen = even_ceiling(even_screen_size / 2) as u8;
        let frame_duration = Duration::from_millis(ONE_SECOND_MILLIS / frames_per_second);

        let snake = Snake::new(half_screen, half_screen, screen_width, screen_height,
                               floating_walls_mode, initial_snake_length);
        let food = Food::new_at_random_position(&snake);

        let mut audio = Audio::new();
        audio.add("audio_move", "audio/audio_move.wav");
        audio.add("audio_capture", "audio/audio_capture.wav");
        audio.add("audio_lose", "audio/audio_lose.wav");

        Renderer {
            screen_width,
            screen_height,
            snake,
            food,
            frame_duration,
            input_receiver,
            captured_food: Vec::new(),
            score: 0,
            game_over: false,
            pause: false,
            floating_walls_mode,
            audio
        }
    }

    pub fn main_loop(&mut self) {
        display::init(self.screen_height, self.screen_width, self.floating_walls_mode);

        loop {
            sleep(self.frame_duration);

            let message = self.input_receiver.try_recv();

            if let Ok(Command::EXIT) = message { break }
            if let Ok(Command::PAUSE) = message { self.pause = !self.pause; continue; }

            if self.game_over || self.pause { continue }

            if let Ok(command) = message {
                if self.snake.command(command) { self.audio.play("audio_move"); }
            }

            if self.refresh_positions().is_err() { self.audio.play("audio_lose"); self.game_over = true; continue; }

            display::draw_food(&self.food);
            display::draw_snake(&self.snake);
            display::draw_score(self.score);
        }
    }

    fn refresh_positions(&mut self) -> Result<(), ColisionEvent> {
        let initial_tail = self.snake.tail();
        if self.snake.tail_collides(self.captured_food.first()) {
            self.snake.grow_from_tail();
            self.captured_food.remove(0);
        } else {
            display::erase_at_position(initial_tail.y, initial_tail.x);
        }

        self.snake.proceed();

        if self.snake.head_collides_with_body() { return Err(CollidedWithBody); }
        if self.snake.head_collides_with_fence() { return Err(CollidedWithFence); }

        if self.snake.head_collides(&self.food.pos) {
            self.audio.play("audio_capture");
            self.captured_food.push(self.food.pos.clone());
            self.food = Food::new_at_random_position(&self.snake);
            self.score += 1;
        }

        return Ok(());
    }
}
