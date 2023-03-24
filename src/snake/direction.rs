#[derive(Clone, PartialEq)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

impl Direction {
    pub fn to_string(&self) -> char {
        match self {
            Direction::UP => 'o',
            Direction::DOWN => 'o',
            Direction::LEFT => 'o',
            Direction::RIGHT => 'o',
        }
    }
}
