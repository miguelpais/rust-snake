use super::direction::Direction;

#[derive(Clone)]
pub struct Point {
    pub x: u16,
    pub y: u16,
    pub direction: Direction
}

impl Point {
    pub fn move_to(&mut self, direction: Direction) {
        self.direction = direction;
        match self.direction {
            Direction::UP => self.y -= 1,
            Direction::DOWN => self.y += 1,
            Direction::LEFT => self.x -= 2,
            Direction::RIGHT => self.x += 2,
            _ => (),
        }
    }
}
