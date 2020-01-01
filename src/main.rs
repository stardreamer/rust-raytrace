mod structs;

use std::fs::File;
use std::io::{Error, Write};
use structs::vec3::Vec3;

fn main() {
    let mut file = File::create("img.ppm").expect("Unable to create file!");
    let nx = 200;
    let ny = 100;
    writeln!(&mut file, "P3\n {} {} \n255", nx, ny).expect("Unable to write to file!");
    for j in (0..ny).rev() {
        for i in 0..nx {
            let col = Vec3::new((i as f64) / (nx as f64), (j as f64) / (ny as f64), 0.2_f64);
            let ir = (255.99 * col.r()) as i64;
            let ig = (255.99 * col.g()) as i64;
            let ib = (255.99 * col.b()) as i64;
            writeln!(&mut file, "{} {} {}", ir, ig, ib).expect("Unable to write to file");
        }
    }
}
