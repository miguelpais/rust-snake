use std::thread::sleep;
use std::time::Duration;
use std::sync::mpsc::Receiver;

use crate::util::even_ceiling;
use crate::snake::command::Command;
use crate::snake::snake::Snake;
use crate::snake::display;
use crate::snake::beer::Beer;
use crate::snake::point::Point;

const ONE_SECOND_MILLIS: u64 = 1_000;

pub struct Renderer {
    snake: Snake,
    beer: Beer,
    screen_width: u16,
    screen_height: u16,
    frame_duration: Duration,
    input_receiver: Receiver<Command>,
    removed_beers: Vec<Point>,
    score: u16,
}

struct CollidedWithBodyErr;

impl Renderer {
    pub fn new(screen_size: u16, initial_snake_length: u8, frames_per_second: u64, input_receiver: Receiver<Command>) -> Renderer {
        let even_screen_size = even_ceiling(screen_size);
        let screen_width = even_screen_size * 2;
        let screen_height = even_screen_size;
        let half_screen = (even_screen_size / 2) as u8;
        let frame_duration = Duration::from_millis(ONE_SECOND_MILLIS / frames_per_second);

        let snake = Snake::new(half_screen, half_screen, screen_width, screen_height, initial_snake_length);
        let beer = Beer::new_at_random_position(&snake);

        Renderer {
            screen_width,
            screen_height,
            snake,
            beer,
            frame_duration,
            input_receiver,
            removed_beers: Vec::new(),
            score: 0
        }
    }

    pub fn main_loop(&mut self) {
        let mut game_over = false;
        let mut pause = false;
        display::init(self.screen_height, self.screen_width);

        loop {
            sleep(self.frame_duration);
            let previous_tail_pos = self.snake.tail();

            let message = self.input_receiver.try_recv();
            if let Ok(Command::EXIT) = message { break }
            if let Ok(Command::PAUSE) = message { pause = !pause; continue; }
            if game_over || pause { continue }
            if let Ok(command) = message { self.snake.change_direction(command.to_direction()) }

            if let Err(CollidedWithBodyErr) = self.refresh_positions(previous_tail_pos) {
                game_over = true;
                continue;
            }

            display::draw_beer(&self.beer);
            display::draw_snake(&self.snake);
            display::draw_score(self.score);
        }
    }

    fn refresh_positions(&mut self, previous_tail_pos: Point) -> Result<(), CollidedWithBodyErr> {
        if previous_tail_pos.collides_with_first(&self.removed_beers) {
            self.snake.push(previous_tail_pos);
            self.removed_beers.remove(0);
        } else {
            display::erase_at_position(previous_tail_pos.y, previous_tail_pos.x);
        }

        self.snake.proceed();
        let new_head = self.snake.head();

        if self.snake.collided_with_body() {
            return Err(CollidedWithBodyErr);
        }
        if new_head.collides(&self.beer.pos) {
            self.removed_beers.push(self.beer.pos.clone());
            self.beer = Beer::new_at_random_position(&self.snake);
            self.score += 1;
        }

        return Ok(());
    }
}
