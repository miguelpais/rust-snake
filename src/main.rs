mod util;
mod game;
mod snake;

const SCREEN_SIZE:u16 = 26;
const INITIAL_SNAKE_LENGTH: u8 = 10;
const FRAMES_PER_SECOND: u64 = 8;
const FLOATING_WALLS: bool = true;

fn main() {
    game::start(SCREEN_SIZE,
                INITIAL_SNAKE_LENGTH,
                FRAMES_PER_SECOND,
                FLOATING_WALLS);
}
