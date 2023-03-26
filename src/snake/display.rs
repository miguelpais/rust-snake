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

pub fn init() {
    let mut stdout = io::stdout();
    stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
    stdout.queue(Clear(ClearType::All)).unwrap();
    stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
    stdout.queue(SetForegroundColor(Color::DarkYellow)).unwrap();
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

pub fn draw_fence(start_y: u16, y_max: u16, start_x: u16, x_max: u16) {
    let mut stdout = io::stdout();

    stdout.queue(MoveTo(start_x, start_y)).unwrap();
    print!("{}", "┌");
    stdout.queue(MoveTo(x_max, start_y)).unwrap();
    print!("{}", "┐");
    stdout.queue(MoveTo(start_x, y_max)).unwrap();
    print!("{}", "└");
    stdout.queue(MoveTo(x_max, y_max)).unwrap();
    print!("{}", "┘");

    for i in 1..x_max {
        stdout.queue(MoveTo(i, start_y)).unwrap();
        print!("{}", "─");
        stdout.queue(MoveTo(i, y_max)).unwrap();
        print!("{}", "─");
    }

    for i in 1..y_max {
        stdout.queue(MoveTo(start_x, i)).unwrap();
        print!("{}", "│");
        stdout.queue(MoveTo(x_max, i)).unwrap();
        print!("{}", "│");
    }
}

pub fn erase_at_position(x: u16, y: u16) {
    let mut stdout = io::stdout();

    stdout.queue(MoveTo(x, y)).unwrap();
    print!(" ");
}