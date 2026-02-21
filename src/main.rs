use std::{
    io::{self, Write},
    thread::sleep,
    time::Duration,
};

const WIDTH: usize = 80;
const HEIGHT: usize = 25;

const DISTANCE: f32 = 60.0;
const SCALE: f32 = 30.0;
const Y_SCALE: f32 = 0.5;

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

struct Ang {
    ax: f32,
    ay: f32,
    az: f32,
}

fn rotate(x: f32, y: f32, z: f32, a: &Ang) -> (f32, f32, f32) {
    let (sx, cx) = a.ax.sin_cos();
    let (sy, cy) = a.ay.sin_cos();
    let (sz, cz) = a.az.sin_cos();

    // X
    let y1 = y * cx - z * sx;
    let z1 = y * sx + z * cx;
    let x1 = x;

    // Y
    let x2 = x1 * cy + z1 * sy;
    let z2 = -x1 * sy + z1 * cy;
    let y2 = y1;

    // Z
    let x3 = x2 * cz - y2 * sz;
    let y3 = x2 * sz + y2 * cz;

    (x3, y3, z2)
}

fn main() {
    print!("\x1b[2J\x1b[HEIGHT");
    let mut frame = Frame::new();
    let mut ang = Ang { ax: 0.0, ay: 0.0, az: 0.0 };

    loop {
        frame.clear();

        // Just a test :)
        let (xr, yr, zr) = rotate(10.0, 10.0, 10.0, &ang);
        let z_cam = zr + DISTANCE;
        if z_cam > 0.0 {
            let inv = 1.0 / z_cam;
            let xp = (WIDTH as f32 / 2.0 + xr * inv * SCALE) as isize;
            let yp = (HEIGHT as f32 / 2.0 - yr * inv * SCALE * Y_SCALE) as isize;
            frame.put(xp, yp, b'*');
        }

        frame.flush();

        ang.ax += 0.05;
        ang.ay += 0.04;
        ang.az += 0.03;

        sleep(Duration::from_millis(30));
    }
}