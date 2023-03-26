use crate::snake::direction::Direction;
use crate::snake::point::Point;

pub struct Snake {
    pub pos: Vec<Point>,
    pub length: u16,
    pub screen_height: u16,
    pub screen_width: u16,
    pub direction: Direction,
}

impl Snake {
    pub fn new(x_start: u8, y_start: u8, screen_width: u16, screen_height: u16, initial_snake_length: u8) -> Snake {

        Snake {
            pos: (0..initial_snake_length).map(|el| Point {
                y: y_start as u16,
                x: (x_start + el * 2) as u16,
            }).collect(),
            length: initial_snake_length as u16,
            screen_height,
            screen_width,
            direction: Direction::LEFT
        }
    }

    pub fn change_direction(&mut self, direction: Direction) {
        match direction {
            Direction::UP => if self.direction != Direction::DOWN { self.direction = Direction::UP },
            Direction::DOWN => if self.direction != Direction::UP { self.direction = Direction::DOWN },
            Direction::LEFT => if self.direction != Direction::RIGHT { self.direction = Direction::LEFT },
            Direction::RIGHT => if self.direction != Direction::LEFT { self.direction = Direction::RIGHT },
            _ => (),
        }
    }

    pub fn proceed(&mut self) {
        let mut previous_body_part_pos = self.get_and_update_head();

        for idx in 1..self.length as usize {
            previous_body_part_pos = self.update_and_get_body_part(idx, previous_body_part_pos);
        }
    }

    fn get_and_update_head(&mut self) -> Point {
        let mut head_pos = &mut self.pos[0];
        let previous_head = Point {
            y: head_pos.y,
            x: head_pos.x
        };

        head_pos.move_to(&self.direction);

        if head_pos.y >= self.screen_height {
            head_pos.y = 1;
        } else if head_pos.y < 1 {
            head_pos.y = self.screen_height - 1;
        }
        if head_pos.x >= self.screen_width {
            head_pos.x = 2
        } else if head_pos.x < 2 {
            head_pos.x = self.screen_width - 2
        }

        previous_head
    }

    fn update_and_get_body_part(&mut self, idx: usize, previous_body_part_pos: Point) -> Point {
        let new_previous_pos = Point {
            y: self.pos[idx].y,
            x: self.pos[idx].x,
        };
        self.pos[idx].x = previous_body_part_pos.x;
        self.pos[idx].y = previous_body_part_pos.y;

        new_previous_pos
    }
}
