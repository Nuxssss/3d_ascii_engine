use std::cmp::max;
use std::io::{Read, stdin, stdout, Write};

use termion::async_stdin;

use crate::cube::Cube;
use crate::point3d::Point3D;

mod bivector3;
mod point3d;
mod cube;
mod vector3d;

type Angle = Point3D;


struct Camera {
    coordinate: Point3D,
    angle: Angle,
    fov: f64,
}

impl Camera {
    fn ray_cast(&self, variance: (i32, i32), cube: Cube) -> Option<i32> {
        let mut start_point = self.coordinate;
        let compare_helper = Point3D::new(2, 2, 2);

        for i in 0..50 {
            let ray_position = start_point + Point3D::new(1, 1, 1);
            for &point in cube.plot().iter() {
                if (ray_position - point).abs() > compare_helper {
                    return Some(i);
                };
            }
        }
        None
    }
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

    let camera = Camera {
        coordinate: Point3D::new(20, 20, 5),
        angle: Angle::new(0, 0, 0),
        fov: 75.0,
    };

    let cube = Cube {
        coordinate: Point3D::new(0, 0, 0),
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
                    camera.fov / term_width as f64 * term_height as f64,
                );
            for x in 1..term_width + 1 {
                let x_ray_angle =
                    calc_ray_angle(
                        x,
                        half_width,
                        width_is_odd,
                        camera.fov,
                    );
                //А вот тут уже можно работать с лучами

                println!("y{} x_angle {} y_angle {}", y, x_ray_angle, y_ray_angle)
            }
        }
    }
}
