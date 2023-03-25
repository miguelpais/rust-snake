use std::io;

use crossterm::{
    cursor::MoveTo,
    QueueableCommand,
};

pub fn draw(start_y: u16, y_max: u16, start_x: u16, x_max: u16) {
    let mut stdout = io::stdout();

    stdout.queue(MoveTo(start_x, start_y)).unwrap();
    print!("{}", "┌");
    stdout.queue(MoveTo(x_max, start_y)).unwrap();
    print!("{}", "┐");
    stdout.queue(MoveTo(start_x, y_max)).unwrap();
    print!("{}", "└");
    stdout.queue(MoveTo(x_max, y_max)).unwrap();
    print!("{}", "┘");

    for i in 1..x_max {
        stdout.queue(MoveTo(i, start_y)).unwrap();
        print!("{}", "─");
        stdout.queue(MoveTo(i, y_max)).unwrap();
        print!("{}", "─");
    }

    for i in 1..y_max {
        stdout.queue(MoveTo(start_x, i)).unwrap();
        print!("{}", "│");
        stdout.queue(MoveTo(x_max, i)).unwrap();
        print!("{}", "│");
    }
}
