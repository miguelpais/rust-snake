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

    pub fn collides(&self, another: &Point) -> bool {
        return self.x == another.x && self.y == another.y
    }

    pub fn collides_with_first(&self, points_vector: &Vec<Point>) -> bool {
        if points_vector.is_empty() { return false }

        points_vector[0].collides(self)
    }
}
