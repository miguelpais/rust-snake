# rust-snake
A snake game built in Rust running inside the terminal window.

![alt text](https://github.com/miguelpais/rust-snake/blob/main/screen.png)

## How to run

```terminal
$ cargo build
$ cargo run
```

Play with the arrows keys, pause with SPACEBAR key and exit with ESC.

## Configuration

Change the following main.rs files in order to adapt the game to your needs.

- SCREEN_SIZE: can be any value and will create a square of the desired size;
- INITIAL_SNAKE_LENGTH: number of parts the initial snake is composed of;
- FRAMES_PER_SECOND: slows or increases the snake speed;
- FLOATING_WALLS: if we desired the snake to teletransport through walls
