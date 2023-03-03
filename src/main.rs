use rand::prelude::*;
use std::io::Write;

type Color = u32;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

static mut IMAGE: [[Color; WIDTH]; HEIGHT] = [[0x0; WIDTH]; HEIGHT];

const COLOR_WHITE: Color = 0xFFFFFFFF;
const COLOR_BLACK: Color = 0x00000000;
const COLOR_RED: Color = 0xFFFF0000;

const MARKER_COLOR: Color = COLOR_BLACK;
const BACKGROUND_COLOR: Color = 0xFF333333;

const PALETTE_SIZE: usize = 10;
const COLOR_PALETTE: [Color; PALETTE_SIZE] = [
    0xFFDFFF00, 0xFFFFBF00, 0xFFFF7F50, 0xFFDE3163, 0xFF9FE2BF, 0xFF40E0D0, 0xFF6495ED, 0xFFCCCCFF,
    0xFF3355FF, 0xFFFF33FF,
];

const NUMBER_OF_MARKERS: usize = 20;
const MARKER_RADIUS: usize = 5;

static mut MARKERS: [Point; NUMBER_OF_MARKERS] = [Point::const_default(); NUMBER_OF_MARKERS];

#[derive(Default, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    const fn const_default() -> Self {
        Self { x: 0, y: 0 }
    }
}

fn main() {
    fill_image(BACKGROUND_COLOR);
    generate_random_markers();
    render_diagram();
    fill_markers();
    save_as_ppm("diagram.ppm");
}

fn fill_image(color: Color) {
    unsafe {
        IMAGE.iter_mut().for_each(|y| {
            y.iter_mut().for_each(|x| {
                *x = color;
            })
        })
    }
}

fn save_as_ppm(filename: &str) {
    let mut file = std::fs::File::create(filename).expect("unable to open or create file");
    write!(file, "P6\n{WIDTH} {HEIGHT}\n255\n")
        .expect(format!("unable to write into file: {filename}").as_str());
    unsafe {
        for y in &IMAGE {
            for x in y {
                let bytes: [u8; 3] = [
                    ((x & 0x00FF0000) >> 8 * 2) as u8,
                    ((x & 0x00FF00) >> 8) as u8,
                    (x & 0x00FF) as u8,
                ];
                file.write_all(&bytes).expect("failed to write file");
            }
        }
    }
    file.flush().expect("unable to flush file");
}

fn generate_random_markers() {
    for i in 0..NUMBER_OF_MARKERS {
        let x = (random::<usize>() % WIDTH) as i32;
        let y = (random::<usize>() % HEIGHT) as i32;

        unsafe { MARKERS[i] = Point { x, y } }
    }
}

fn fill_markers() {
    unsafe {
        for marker in &MARKERS {
            fill_circle(marker, MARKER_RADIUS, MARKER_COLOR);
        }
    }
}

fn fill_circle(point: &Point, radius: usize, color: Color) {
    let x0 = point.x - radius as i32;
    let x1 = point.x + radius as i32;
    let y0 = point.y - radius as i32;
    let y1 = point.y + radius as i32;
    for x in x0..x1 {
        if x >= 0 && (x as usize) < WIDTH {
            for y in y0..y1 {
                if y >= 0 && (y as usize) < HEIGHT {
                    if sqr_dist(point.x, x, point.y, y) <= radius.wrapping_mul(radius) {
                        unsafe {
                            IMAGE[y as usize][x as usize] = color;
                        }
                    }
                }
            }
        }
    }
}

fn render_diagram() {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let mut j = 0usize;
            for i in 1..NUMBER_OF_MARKERS {
                unsafe {
                    if sqr_dist(MARKERS[i].x, x as i32, MARKERS[i].y, y as i32)
                        < sqr_dist(MARKERS[j].x, x as i32, MARKERS[j].y, y as i32)
                    {
                        j = i;
                    }
                }
            }
            unsafe {
                IMAGE[y][x] = COLOR_PALETTE[j % PALETTE_SIZE];
            }
        }
    }
}

fn sqr_dist(x: i32, x1: i32, y: i32, y1: i32) -> usize {
    let dx = x - x1;
    let dy = y - y1;
    (dx * dx + dy * dy) as usize
}
