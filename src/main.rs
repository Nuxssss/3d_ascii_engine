use std::io::{Read, stdin, stdout, Write};

use termion::async_stdin;

struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

struct Cube {
    coordinate: Point3D,
    size: i32,
}

fn calc_ray_angle(num: i8, half_size: i8, size_is_odd: bool, fov: f64) -> f64 {
    let n =
        if size_is_odd {
            if num <= half_size { num - half_size - 1 } else { num - half_size }
        } else {
            num - half_size - 1
        };
    n as f64 / half_size as f64 * fov
}

fn main() {
    //let stdout = stdout();
    //let mut stdout = stdout.lock().into_raw_mode().unwrap();
    //let mut stdin = async_stdin().bytes();
    let fov = 75.0;
    let cube = Cube {
        coordinate: Point3D {
            x: 0,
            y: 0,
            z: 0,
        },
        size: 5,
    };
    loop {
        let (term_width, term_height) = termion::terminal_size().unwrap();
        let (term_width, term_height) = (term_width as i8, term_height as i8);
        let (width_is_odd, height_is_odd) = (term_width % 2 == 0, term_height % 2 == 0);
        let (half_width, half_height) = (term_width / 2, term_height / 2);
        for y in 1..term_height + 1 {
            let y_ray_angle =
                calc_ray_angle(
                    y,
                    half_height,
                    height_is_odd,
                    fov / term_width * term_height,
                );
            for x in 1..term_width + 1 {
                let x_ray_angle =
                    calc_ray_angle(
                        x,
                        half_width,
                        width_is_odd,
                        fov,
                    );
                //println!("y{} x_angle {} y_angle {}", y, x_ray_angle, y_ray_angle)
            }
        }
    }
}
