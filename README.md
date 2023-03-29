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

- `SCREEN_SIZE`: will create a fence of the desired size;
- `INITIAL_SNAKE_LENGTH`: number of parts the initial snake is composed of;
- `FRAMES_PER_SECOND`: number of renders occuring in a second, with each render corresponding to a snake position change. In pratice this slows or increases the snake speed;
- `FLOATING_WALLS`: if the snake should teletransport through walls instead of colliding. Floating and Solid walls are rendered slightly differently.
