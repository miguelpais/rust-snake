use crate::snake::direction::Direction;
use crate::snake::command::Command;
use crate::snake::point::Point;

pub struct Snake {
    pub pos: Vec<Point>,
    pub screen_height: u16,
    pub screen_width: u16,
    pub direction: Direction,
    floating_walls_mode: bool,
}

impl Snake {
    pub fn new(x_start: u8, y_start: u8, screen_width: u16, screen_height: u16, floating_walls_mode: bool, initial_snake_length: u8) -> Snake {

        Snake {
            pos: (0..initial_snake_length).map(|el| Point {
                y: y_start as u16,
                x: (x_start + el * 2) as u16,
            }).collect(),
            screen_height,
            screen_width,
            floating_walls_mode,
            direction: Direction::LEFT
        }
    }

    pub fn command(&mut self, command: Command) -> bool {
        let initial_direction = self.direction.clone();
        match command {
            Command::UP => if self.direction != Direction::DOWN { self.direction = Direction::UP },
            Command::DOWN => if self.direction != Direction::UP { self.direction = Direction::DOWN },
            Command::LEFT => if self.direction != Direction::RIGHT { self.direction = Direction::LEFT },
            Command::RIGHT => if self.direction != Direction::LEFT { self.direction = Direction::RIGHT },
            _ => (),
        }

        return initial_direction != self.direction;
    }

    pub fn proceed(&mut self) {
        let mut previous_body_part_pos = self.get_and_update_head();

        for idx in 1..self.pos.len() {
            previous_body_part_pos = self.update_and_get_body_part(idx, previous_body_part_pos);
        }
    }

    pub fn head(&self) -> Point {
        self.pos.get(0).unwrap().clone()
    }

    pub fn tail(&self) -> Point {
        self.pos.last().unwrap().clone()
    }

    pub fn grow_from_tail(&mut self) {
        self.pos.push(self.tail());
    }

    pub fn head_collides(&self, point: &Point) -> bool {
        return Self::collides_with_any(&self.head(), &vec![point.clone()])
    }

    pub fn tail_collides(&self, opt_point: Option<&Point>) -> bool {
        if opt_point.is_none() { return false }

        return Self::collides_with_any(&self.tail(), &vec![opt_point.unwrap().clone()])
    }

    pub fn head_collides_with_fence(&self) -> bool {
        if self.floating_walls_mode { return false };

        self.head().collides_with_fence(self.screen_height, self.screen_width)
    }

    pub fn head_collides_with_body(&self) -> bool {
        return Self::collides_with_any(&self.head(), &Vec::from(&self.pos[1..]));
    }

    fn collides_with_any(target: &Point, points: &Vec<Point>) -> bool {
        if points.is_empty() { return false };

        for point in points {
            if target.collides(point) { return true }
        }
        return false;
    }

    pub fn present_at(&self, given: &Point) -> bool {
        for point in &self.pos {
            if given.collides(point) { return true }
        }
        return false;
    }

    fn get_and_update_head(&mut self) -> Point {
        let head_pos = &mut self.pos[0];
        let previous_head = Point {
            y: head_pos.y,
            x: head_pos.x
        };

        head_pos.move_to(&self.direction);

        if self.floating_walls_mode {
            head_pos.teletransport(self.screen_height, self.screen_width);
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
