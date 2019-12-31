use std::fs::File;
use std::io::{Error, Write};

fn main() {
    let mut file = File::create("img.ppm").expect("Unable to create file!");
    let nx = 200;
    let ny = 100;
    writeln!(&mut file, "P3\n {} {} \n255", nx, ny).expect("Unable to write to file!");
    for j in (0..ny).rev() {
        for i in 0..nx {
            println!("{} {}", i, j);
            let r = (i as f64) / (nx as f64);
            let g = (j as f64) / (ny as f64);
            let b = 0.2_f64;

            let ir = (255.99 * r) as i64;
            let ig = (255.99 * g) as i64;
            let ib = (255.99 * b) as i64;
            writeln!(&mut file, "{} {} {}", ir, ig, ib).expect("Unable to write to file");
        }
    }
    println!("Hello, world!");
}
