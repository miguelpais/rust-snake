use crate::snake::point::Point;

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
}
