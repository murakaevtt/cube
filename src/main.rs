use std::{
    io::{self, Write},
    thread::sleep,
    time::Duration,
};

const WIDTH: usize = 80;
const HEIGHT: usize = 25;

const DISTANCE: f32 = 40.0;   // расстояние до камеры
const SCALE: f32 = 40.0;  // масштаб
const Y_SCALE: f32 = 0.5; // поправка по вертикали под шрифт
const STEP: f32 = 0.7;    // шаг по поверхности куба
const SIZE: f32 = 8.0;    // полуразмер куба

// символы для граней
const FRONT: u8 = b'%';
const RIGHT: u8 = b'&';
const LEFT:  u8 = b'=';
const BACK:  u8 = b'*';
const BOTTOM:u8 = b'^';
const TOP:   u8 = b'+';

struct Ang {
    ax: f32,
    ay: f32,
    az: f32,
}

struct Frame {
    buf:   Vec<u8>,
    depth: Vec<f32>,
}

impl Frame {
    fn new() -> Self {
        Self {
            buf:   vec![b' '; WIDTH * HEIGHT],
            depth: vec![0.0; WIDTH * HEIGHT],
        }
    }

    fn clear(&mut self) {
        self.buf.fill(b' ');
        self.depth.fill(0.0);
    }

    fn put(&mut self, x: isize, y: isize, z_inv: f32, ch: u8) {
        if x < 0 || y < 0 || x >= WIDTH as isize || y >= HEIGHT as isize {
            return;
        }
        let i = y as usize * WIDTH + x as usize;
        if z_inv > self.depth[i] {
            self.depth[i] = z_inv;
            self.buf[i] = ch;
        }
    }

    fn flush(&self) {
        print!("\x1b[HEIGHT");
        let mut out = io::stdout();
        for y in 0..HEIGHT {
            let start = y * WIDTH;
            out.write_all(&self.buf[start..start + WIDTH]).unwrap();
            out.write_all(b"\n").unwrap();
        }
        out.flush().unwrap();
    }
}

fn rotate(x: f32, y: f32, z: f32, a: &Ang) -> (f32, f32, f32) {
    let (sx, cx) = a.ax.sin_cos();
    let (sy, cy) = a.ay.sin_cos();
    let (sz, cz) = a.az.sin_cos();

    // вокруг X
    let y1 = y * cx - z * sx;
    let z1 = y * sx + z * cx;
    let x1 = x;

    // вокруг Y
    let x2 = x1 * cy + z1 * sy;
    let z2 = -x1 * sy + z1 * cy;
    let y2 = y1;

    // вокруг Z
    let x3 = x2 * cz - y2 * sz;
    let y3 = x2 * sz + y2 * cz;

    (x3, y3, z2)
}

fn plot_point(x: f32, y: f32, z: f32, ang: &Ang, f: &mut Frame, ch: u8) {
    let (xr, yr, zr) = rotate(x, y, z, ang);
    let z_cam = zr + DISTANCE;
    if z_cam <= 0.0 {
        return;
    }

    let inv = 1.0 / z_cam;
    let xp = (WIDTH as f32 / 2.0 + xr * inv * SCALE) as isize;
    let yp = (HEIGHT as f32 / 2.0 - yr * inv * SCALE * Y_SCALE) as isize;

    f.put(xp, yp, inv, ch);
}

fn main() {
    print!("\x1b[2J\x1b[HEIGHT");
    let mut ang = Ang { ax: 0.0, ay: 0.0, az: 0.0 };
    let mut frame = Frame::new();

    loop {
        frame.clear();

        let mut x = -SIZE;
        while x <= SIZE {
            let mut y = -SIZE;
            while y <= SIZE {
                // 6 граней куба, у каждой свой символ
                plot_point(x,      y,     -SIZE, &ang, &mut frame, FRONT);  // front
                plot_point(SIZE,   y,      x,    &ang, &mut frame, RIGHT);  // right
                plot_point(-SIZE,  y,     -x,    &ang, &mut frame, LEFT);   // left
                plot_point(-x,     y,      SIZE, &ang, &mut frame, BACK);   // back
                plot_point(x,     -SIZE,  -y,    &ang, &mut frame, BOTTOM); // bottom
                plot_point(x,      SIZE,   y,    &ang, &mut frame, TOP);    // top

                y += STEP;
            }
            x += STEP;
        }

        frame.flush();

        ang.ax += 0.05;
        ang.ay += 0.05;
        ang.az += 0.01;

        sleep(Duration::from_millis(16));
    }
}