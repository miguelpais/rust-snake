use super::direction::Direction;
use super::point::Point;

pub struct Snake {
    pub pos: Vec<Point>,
    pub length: u16,
    screen_height: u16,
    screen_width: u16
}

impl Snake {
    pub fn new(x_start: u8, y_start: u8, screen_width: u16, screen_height: u16, initial_snake_length: u8) -> Snake {

        Snake {
            pos: (0..initial_snake_length).map(|el| Point {
                y: y_start as u16,
                x: (x_start + el * 2) as u16,
                direction: Direction::LEFT
            }).collect(),
            length: initial_snake_length as u16,
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
        let mut head_pos = &mut self.pos[0];
        let previous_head = Point {
            y: head_pos.y,
            x: head_pos.x,
            direction: Direction::NONE
        };

        head_pos.move_to(direction.clone());

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
            direction: Direction::NONE
        };
        self.pos[idx].x = previous_body_part_pos.x;
        self.pos[idx].y = previous_body_part_pos.y;
        self.pos[idx].direction = Direction::NONE;

        new_previous_pos
    }
}
