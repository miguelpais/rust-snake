use std::io;
use std::io::Write;

use crossterm::{
    cursor::MoveTo,
    style::{Color, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType},
    QueueableCommand,
};

use crate::snake::snake::Snake;
use crate::snake::direction::Direction;
use crate::snake::food::Food;
use crate::snake::fence::{Fence, FenceRenderer};

pub fn init(screen_height: u16, screen_width: u16, floating_walls_mode: bool) {
    let mut stdout = io::stdout();
    stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
    stdout.queue(Clear(ClearType::All)).unwrap();
    stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
    stdout.queue(SetForegroundColor(Color::DarkYellow)).unwrap();

    let fence_renderer = Fence::renderer(floating_walls_mode);

    draw_fence(0, screen_height, 0, screen_width, fence_renderer);
}

pub fn draw_snake(snake: &Snake) {
    let mut stdout = io::stdout();
    let snake_parts = &snake.pos;

    stdout.queue(MoveTo(snake_parts[0].x, snake_parts[0].y)).unwrap();
    print!("{}", snake.direction.to_string());

    for pos in &snake.pos[1..] {
        stdout.queue(MoveTo(pos.x, pos.y)).unwrap();
        print!("{}", Direction::NONE.to_string());
    }
    stdout.queue(MoveTo(snake.screen_width + 1, snake.screen_height + 1)).unwrap();
    stdout.flush().unwrap();
}

pub fn draw_score(score: u16) {
    let mut stdout = io::stdout();
    stdout.queue(MoveTo(0, 100)).unwrap();
    print!("Score: {}", score);
}

pub fn draw_food(beer: &Food) {
    let mut stdout = io::stdout();
    stdout.queue(MoveTo(beer.pos.x, beer.pos.y)).unwrap();
    print!("{}", beer.to_string());
}

pub fn draw_fence(start_y: u16, y_max: u16, start_x: u16, x_max: u16, fence_renderer: Box<dyn FenceRenderer>) {
    let mut stdout = io::stdout();

    stdout.queue(MoveTo(start_x, start_y)).unwrap();
    print!("{}", fence_renderer.top_left());
    stdout.queue(MoveTo(x_max, start_y)).unwrap();
    print!("{}", fence_renderer.top_right());
    stdout.queue(MoveTo(start_x, y_max)).unwrap();
    print!("{}", fence_renderer.bottom_left());
    stdout.queue(MoveTo(x_max, y_max)).unwrap();
    print!("{}", fence_renderer.bottom_right());

    for i in 1..x_max {
        stdout.queue(MoveTo(i, start_y)).unwrap();
        print!("{}", fence_renderer.horizontal_wall());
        stdout.queue(MoveTo(i, y_max)).unwrap();
        print!("{}", fence_renderer.horizontal_wall());
    }

    for i in 1..y_max {
        stdout.queue(MoveTo(start_x, i)).unwrap();
        print!("{}", fence_renderer.vertical_wall());
        stdout.queue(MoveTo(x_max, i)).unwrap();
        print!("{}", fence_renderer.vertical_wall());
    }
}

pub fn erase_at_position(y: u16, x: u16) {
    let mut stdout = io::stdout();

    stdout.queue(MoveTo(x, y)).unwrap();
    print!(" ");
}
