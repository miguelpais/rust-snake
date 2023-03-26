mod util;
mod game;
mod snake;

const SCREEN_SIZE:u16 = 40;
const INITIAL_SNAKE_LENGTH: u8 = 10;
const FRAMES_PER_SECOND: u64 = 8;
const INPUT_CAPTURING_WINDOW_MS: u64 = 3;

fn main() {
    game::start(SCREEN_SIZE,
                INITIAL_SNAKE_LENGTH,
                FRAMES_PER_SECOND,
                INPUT_CAPTURING_WINDOW_MS);
}
