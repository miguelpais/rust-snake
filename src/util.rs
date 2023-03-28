use rand::Rng;

use crate::snake::snake::Snake;
use crate::snake::point::Point;

pub fn even_ceiling(num: u16) -> u16 {
    num +  if num % 2 == 0 { 0 } else { 1 }
}

pub fn get_random_free_point(snake: &Snake) -> Point {
    let mut occupied_points: Vec<&Point> = Vec::new();
    let mut available_points: Vec<Point> = Vec::new();

    for el in &snake.pos {
        occupied_points.push(el);
    }

    for x in (2..snake.screen_width).step_by(2) {
        for y in 1..snake.screen_height {
            let mut occupied = false;
            for occupied_point in occupied_points.clone() {
                if occupied_point.x == x && occupied_point.y == y { occupied = true }
            }
            if !occupied { available_points.push(Point { x, y} ); }
        }
    }
    let mut rgn = rand::thread_rng();
    let random_index = rgn.gen_range(0..available_points.len());

    available_points[random_index].clone()
}
