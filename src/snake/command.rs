use crate::snake::direction::Direction;

#[derive(PartialEq, Clone, Copy)]
pub enum Command {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    EXIT
}

impl Command {
    pub fn to_direction(&self) -> Direction {
        match self {
            Command::UP => Direction::UP,
            Command::DOWN => Direction::DOWN,
            Command::LEFT => Direction::LEFT,
            Command::RIGHT => Direction::RIGHT,
            _ => Direction::NONE,
        }
    }
}
