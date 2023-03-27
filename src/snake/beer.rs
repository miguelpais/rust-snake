use crate::util::get_random_free_point;
use crate::snake::point::Point;
use crate::snake::snake::Snake;

pub struct Beer {
    pub pos: Point
}

impl Beer {
    pub fn new(y: u16, x: u16) -> Beer {
        Beer {
            pos: Point {
                y,
                x
            }
        }
    }

    pub fn new_at_random_position(snake: &Snake) -> Beer {
        let new_random_free_point = get_random_free_point(snake);

        Beer::new(new_random_free_point.y, new_random_free_point.x)
    }

    pub fn to_string(&self) -> &str {
        "🍺"
    }
}
