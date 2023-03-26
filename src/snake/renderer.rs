use std::thread::sleep;
use std::time::Duration;
use std::sync::mpsc::Receiver;
use rand::Rng;

use crate::util::even_ceiling;
use crate::snake::command::Command;
use crate::snake::snake::Snake;
use crate::snake::display;
use crate::snake::beer::Beer;
use crate::snake::point::Point;

const ONE_SECOND_MILIS: u64 = 1_000;

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

impl Renderer {
    pub fn new(screen_size: u16, initial_snake_length: u8, frames_per_second: u64, input_receiver: Receiver<Command>) -> Renderer {
        let even_screen_size = even_ceiling(screen_size);
        let screen_width = even_screen_size * 2;
        let screen_height = even_screen_size;
        let half_screen = (even_screen_size / 2) as u8;
        let frame_duration = Duration::from_millis(ONE_SECOND_MILIS / frames_per_second);

        let snake = Snake::new(half_screen, half_screen, screen_width, screen_height, initial_snake_length);
        let random_point = Self::get_random_free_point_from_snake_positions(&snake, screen_width, screen_height);
        let beer = Beer::new(random_point.y, random_point.x);

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
        display::init();
        display::draw_fence(0, self.screen_height, 0, self.screen_width);

        loop {
            let initial_tail = self.snake.pos.last().unwrap().clone();

            let message = self.input_receiver.try_recv();
            if let Ok(Command::EXIT) = message { break }
            if let Ok(Command::PAUSE) = message { pause = !pause; continue; }

            if !game_over && !pause {
                if let Ok(command) = message { self.snake.change_direction(command.to_direction()) }

                if !self.removed_beers.is_empty() && self.removed_beers[0].collides(&initial_tail) {
                    self.snake.pos.push(initial_tail);
                    self.removed_beers.remove(0);
                } else {
                    display::erase_at_position(initial_tail.y, initial_tail.x);
                }

                self.snake.proceed();

                if self.snake.collided_with_body() {
                    game_over = true;
                    continue;
                }
                if Self::collided(&self.snake, &self.beer) {
                    self.removed_beers.push(self.beer.pos.clone());
                    self.beer = self.generate_new_beer();
                    self.score += 1;
                }


                display::draw_beer(&self.beer);
                display::draw_snake(&self.snake);
                display::draw_score(self.score);
            }

            sleep(self.frame_duration);
        }
    }

    fn collided(snake: &Snake, beer: &Beer) -> bool {
        return snake.present_at(&beer.pos)
    }

    fn generate_new_beer(&mut self) -> Beer {
        let new_random_free_point = self.get_random_free_point();

        Beer::new(new_random_free_point.y, new_random_free_point.x)
    }

    fn get_random_free_point(&mut self) -> Point {
        let mut occupied_points: Vec<&Point> = Vec::new();
        for el in &self.snake.pos {
            occupied_points.push(el);
        }

        Self::get_random_free_point_from_occupied(&occupied_points, self.screen_width, self.screen_height)
    }

    fn get_random_free_point_from_snake_positions(snake: &Snake, screen_width: u16, screen_height: u16) -> Point{
        let mut occupied_points: Vec<&Point> = Vec::new();
        for el in &snake.pos {
            occupied_points.push(el);
        }

        Self::get_random_free_point_from_occupied(&occupied_points, screen_width, screen_height)
    }

    fn get_random_free_point_from_occupied(occupied_points: &Vec<&Point>, screen_width: u16, screen_height: u16) -> Point {
        let mut available_points: Vec<Point> = Vec::new();
        for x in (2..screen_width).step_by(2) {
            for y in 2..screen_height {
                let mut occupied = false;
                for occupied_point in occupied_points {
                    if occupied_point.x == x && occupied_point.y == y { occupied = true }
                }
                if !occupied { available_points.push(Point { x, y} ); }
            }
        }
        let mut rgn = rand::thread_rng();
        let random_index = rgn.gen_range(0..available_points.len());

        available_points[random_index].clone();
    }
}
