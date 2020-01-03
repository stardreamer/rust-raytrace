mod structs;

use std::fs::File;
use std::io::{Error, Write};
use std::option;
use structs::ray::Ray;
use structs::vec3::Vec3;

fn hit_sphere(center: Vec3, radius: f64, ray: &Ray) -> Option<f64> {
    let oc = ray.origin - center;
    let a = ray.direction.dot(&ray.direction);
    let b = 2_f64 * oc.dot(&ray.direction);
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = (b * b - 4_f64 * a * c);

    if (discriminant < 0_f64) {
        None
    } else {
        Some((-b - discriminant.sqrt()) / (2_f64 * a))
    }
}

fn color(ray: &Ray) -> Vec3 {
    match hit_sphere(Vec3::new(0_f64, 0_f64, -1_f64), 0.5_f64, ray) {
        Some(t) => {
            let n = (ray.point_at(t) - Vec3::new(0_f64, 0_f64, -1_f64)).unit_vector();
            return 0.5_f64 * Vec3::new(n.x() + 1_f64, n.y() + 1_f64, n.z() + 1_f64);
        }
        None => (),
    }

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
            let (ir, ig, ib) = col.irgb(255.99_f64);
            writeln!(&mut file, "{} {} {}", ir, ig, ib).expect("Unable to write to file");
        }
    }
}
