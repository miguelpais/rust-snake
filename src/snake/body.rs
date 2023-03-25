use super::direction::Direction;
use std::io;
use super::util::even_ceiling;
use crossterm::{
    cursor::MoveTo,
    style::{Color, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType},
    QueueableCommand,
};

pub struct SnakeBody {
    pub pos: Vec<Point>,
    pub length: u16,
    screen_height: u16,
    screen_width: u16
}

#[derive(Clone)]
pub struct Point {
    pub row: u16,
    pub column: u16,
    pub direction: Direction
}

const SNAKE_LENGTH: u16 = 10;

impl SnakeBody {
    pub fn new(row_head: u16, column_head: u16, screen_height: u16, screen_width: u16) -> SnakeBody {

        SnakeBody {
            pos: (0..SNAKE_LENGTH).map(|el| Point {
                row: row_head,
                column: column_head + el * 2,
                direction: Direction::LEFT
            }).collect(),
            length: SNAKE_LENGTH,
            screen_height,
            screen_width
        }
    }

    pub fn update_position(&mut self, direction: &Direction) {
        let mut previous_body_part_pos = self.get_and_update_head(direction);

        for idx in 1..self.length as usize {
            previous_body_part_pos = self.update_and_get_body_part(idx, previous_body_part_pos);
        }
    }

    fn get_and_update_head(&mut self, direction: &Direction) -> Point {
        let head_pos = &mut self.pos[0];
        let mut previous_head = Point {
            row: head_pos.row,
            column: head_pos.column,
            direction: head_pos.direction.clone()
        };

        head_pos.direction = direction.clone();
        match direction {
            Direction::UP => head_pos.row = previous_head.row - 1,
            Direction::DOWN => head_pos.row = previous_head.row + 1,
            Direction::LEFT => head_pos.column = previous_head.column - 2,
            Direction::RIGHT => head_pos.column = previous_head.column + 2,
            _ => (),
        }
        let mut stdout = io::stdout();
        stdout.queue(MoveTo(90, 90)).unwrap();
        print!("{},{}",head_pos.column, head_pos.row);
        if head_pos.row >= self.screen_height {
            head_pos.row = 1;
        } else if head_pos.row < 1 {
            head_pos.row = self.screen_height - 1;
        }
        if head_pos.column >= self.screen_width {
            head_pos.column = 2
        } else if head_pos.column < 2 {
            head_pos.column = self.screen_width - 2
        }

        previous_head
    }

    fn update_and_get_body_part(&mut self, idx: usize, previous_body_part_pos: Point) -> Point {
        let new_previous_pos = Point {
            row: self.pos[idx].row,
            column: self.pos[idx].column,
            direction: Direction::NONE
        };
        self.pos[idx].column = previous_body_part_pos.column;
        self.pos[idx].row = previous_body_part_pos.row;
        self.pos[idx].direction = Direction::NONE;

        new_previous_pos
    }
}
