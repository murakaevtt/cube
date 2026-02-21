use std::{
    io::{self, Write},
    thread::sleep,
    time::Duration,
};

const WIDTH: usize = 80;
const HEIGHT: usize = 25;

fn main() {
    print!("\x1b[2J\x1b[H");
    let mut out = io::stdout();

    loop {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let ch = if y == 0 || y == HEIGHT - 1 || x == 0 || x == WIDTH - 1 {
                    '#'
                } else {
                    ' '
                };
                write!(out, "{}", ch).unwrap();
            }
            writeln!(out).unwrap();
        }
        out.flush().unwrap();

        sleep(Duration::from_millis(200));

        write!(out, "\x1b[H").unwrap();
    }
}
