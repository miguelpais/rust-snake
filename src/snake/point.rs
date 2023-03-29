use crate::snake::direction::Direction;

#[derive(Clone)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

impl Point {
    pub fn move_to(&mut self, direction: &Direction) {
        match direction {
            Direction::UP => self.y -= 1,
            Direction::DOWN => self.y += 1,
            Direction::LEFT => self.x -= 2,
            Direction::RIGHT => self.x += 2,
            _ => (),
        }
    }

    pub fn collides_with_fence(&self, screen_height: u16, screen_width: u16) -> bool {
        return self.y >= screen_height || self.y < 1 || self.x >= screen_width || self.x < 2 ;
    }

    pub fn teletransport(&mut self, screen_height: u16, screen_width: u16) {
            if self.y >= screen_height {
                self.y = 1;
            } else if self.y < 1 {
                self.y = screen_height - 1;
            }
            if self.x >= screen_width {
                self.x = 2
            } else if self.x < 2 {
                self.x = screen_width - 2
            }
    }

    pub fn collides(&self, another: &Point) -> bool {
        return self.x == another.x && self.y == another.y
    }
}
