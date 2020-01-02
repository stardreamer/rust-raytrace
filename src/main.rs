mod structs;

use std::fs::File;
use std::io::{Error, Write};
use structs::ray::Ray;
use structs::vec3::Vec3;

fn color(ray: &Ray) -> Vec3 {
    let unit_direction = ray.direction.unit_vector();

    let t = 0.5_f64 * (unit_direction.y() + 1_f64);

    (1_f64 - t) * Vec3::new(1_f64, 1_f64, 1_f64) + t * Vec3::new(0.5_f64, 0.7_f64, 1.0_f64)
}

fn main() {
    let mut file = File::create("img.ppm").expect("Unable to create file!");
    let nx = 400;
    let ny = 200;
    writeln!(&mut file, "P3\n {} {} \n255", nx, ny).expect("Unable to write to file!");

    let lower_left_corner = Vec3::new(-2_f64, -1_f64, -1_f64);
    let horizontal = Vec3::new(4_f64, 0_f64, 0_f64);
    let vertical = Vec3::new(0_f64, 2_f64, 0_f64);
    let origin = Vec3::default();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = (i as f64) / (nx as f64);
            let v = (j as f64) / (ny as f64);

            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
            let col = color(&r);
            let ir = (255.99 * col.r()) as i64;
            let ig = (255.99 * col.g()) as i64;
            let ib = (255.99 * col.b()) as i64;
            writeln!(&mut file, "{} {} {}", ir, ig, ib).expect("Unable to write to file");
        }
    }
}
