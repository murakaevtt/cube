use std::{
    io::{self, Write},
    thread::sleep,
    time::Duration,
};

const WIDTH: usize = 80;
const HEIGHT: usize = 25;

struct Frame {
    buf: Vec<u8>,
}

impl Frame {
    fn new() -> Self {
        Self {
            buf: vec![b' '; WIDTH * HEIGHT],
        }
    }

    fn clear(&mut self) {
        self.buf.fill(b' ');
    }

    fn put(&mut self, x: isize, y: isize, ch: u8) {
        if x < 0 || y < 0 || x >= WIDTH as isize || y >= HEIGHT as isize {
            return;
        }
        let i = y as usize * WIDTH + x as usize;
        self.buf[i] = ch;
    }

    fn flush(&self) {
        let mut out = io::stdout();
        write!(out, "\x1b[HEIGHT").unwrap();
        for y in 0..HEIGHT {
            let start = y * WIDTH;
            out.write_all(&self.buf[start..start + WIDTH]).unwrap();
            out.write_all(b"\n").unwrap();
        }
        out.flush().unwrap();
    }
}

fn main() {
    print!("\x1b[2J\x1b[HEIGHT");
    let mut frame = Frame::new();

    loop {
        frame.clear();
        
        let cx = (WIDTH / 2) as isize;
        let cy = (HEIGHT / 2) as isize;
        for dx in -5..=5 {
            frame.put(cx + dx, cy, b'-');
            frame.put(cx, cy + dx, b'|');
        }

        frame.flush();
        sleep(Duration::from_millis(100));
    }
}