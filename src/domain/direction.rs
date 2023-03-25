#[derive(Clone, PartialEq)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    NONE,
}

impl Direction {
    pub fn to_string(&self) -> char {
        match self {
            Direction::UP => '▲',
            Direction::DOWN => '▼',
            Direction::LEFT => '◀',
            Direction::RIGHT => '▶',
            Direction::NONE => '●'
        }
    }
}
