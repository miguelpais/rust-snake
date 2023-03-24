use super::direction::Direction;

pub struct SnakeBody {
    pub pos: Vec<Point>,
    pub length: usize,
}

#[derive(Clone)]
pub struct Point {
    pub row: usize,
    pub column: usize,
    pub direction: Direction
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
            Direction::RIGHT => head_pos.column = previous_body_part_pos.column + 2,
            _ => (),
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
                direction: Direction::NONE
            };
            self.pos[idx].column = previous_body_part_pos.column;
            self.pos[idx].row = previous_body_part_pos.row;
            self.pos[idx].direction = Direction::NONE;
            previous_body_part_pos = new_previous_pos;
        }
    }
}
