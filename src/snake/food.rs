use crate::util::get_random_free_point;
use crate::snake::point::Point;
use crate::snake::snake::Snake;

pub struct Food {
    pub pos: Point
}

impl Food {
    pub fn new(y: u16, x: u16) -> Food {
        Food {
            pos: Point {
                y,
                x
            }
        }
    }

    pub fn new_at_random_position(snake: &Snake) -> Food {
        let new_random_free_point = get_random_free_point(snake);

        Food::new(new_random_free_point.y, new_random_free_point.x)
    }

    pub fn to_string(&self) -> &str {
        "🍎"
    }
}
