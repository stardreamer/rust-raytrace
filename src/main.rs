mod materials;
mod obj;
mod random;
mod structs;

use materials::dielectric::Dielectric;
use materials::lambertian::Lambertian;
use materials::metal::Metal;
use materials::Material;
use materials::Scatterable;
use obj::camera::Camera;
use obj::hittablelist::HittableList;
use obj::sphere::Sphere;
use obj::Hittable;
use std::fs::File;
use std::io::Write;
use structs::ray::Ray;
use structs::vec3::Vec3;

fn color(ray: &Ray, world: &HittableList, rng: &mut impl Iterator<Item = f64>, depth: i32) -> Vec3 {
    match world.hit(ray, 0.001_f64, std::f64::MAX) {
        Some(hit) => match hit.material.scatter(ray, &hit, rng) {
            Some((scattered_ray, attenuation)) => {
                if depth > 50 {
                    return Vec3::new(0_f64, 0_f64, 0_f64);
                }

                attenuation * color(&scattered_ray, world, rng, depth + 1)
            }
            None => Vec3::new(0_f64, 0_f64, 0_f64),
        },
        None => {
            let unit_direction = ray.direction.unit_vector();

            let t = 0.5_f64 * (unit_direction.y() + 1_f64);

            (1_f64 - t) * Vec3::new(1_f64, 1_f64, 1_f64) + t * Vec3::new(0.5_f64, 0.7_f64, 1.0_f64)
        }
    }
}

fn main() {
    let mut g1 = random::frand(32);
    let mut g2 = random::frand(44);
    let mut g = random::frand(53);

    let mut file = File::create("img-anti.ppm").expect("Unable to create file!");
    let nx = 400;
    let ny = 200;
    let ns = 100;

    writeln!(&mut file, "P3\n {} {} \n255", nx, ny).expect("Unable to write to file!");

    let camera = Camera::default();

    let world = HittableList {
        objects: vec![
            Box::new(Sphere::new(
                Vec3::new(0_f64, 0_f64, -1_f64),
                0.5,
                Material::Lambertian(Lambertian {
                    albedo: Vec3::new(0.8_f64, 0.3_f64, 0.3_f64),
                }),
            )),
            Box::new(Sphere::new(
                Vec3::new(0_f64, -100.5_f64, -1_f64),
                100_f64,
                Material::Lambertian(Lambertian {
                    albedo: Vec3::new(0.8_f64, 0.8_f64, 0_f64),
                }),
            )),
            Box::new(Sphere::new(
                Vec3::new(1_f64, 0_f64, -1_f64),
                0.5_f64,
                Material::Metal(Metal {
                    albedo: Vec3::new(0.8_f64, 0.8_f64, 0.8_f64),
                    fuzz: 0_f64,
                }),
            )),
            Box::new(Sphere::new(
                Vec3::new(-1_f64, 0_f64, -1_f64),
                0.5_f64,
                Material::Dielectric(Dielectric {
                    reflection_index: 1.5,
                }),
            )),
            Box::new(Sphere::new(
                Vec3::new(-1_f64, 0_f64, -1_f64),
                -0.45_f64,
                Material::Dielectric(Dielectric {
                    reflection_index: 1.5,
                }),
            )),
        ],
    };

    for j in (0..ny).rev() {
        for i in 0..nx {
            let r_rays = g1
                .by_ref()
                .take(ns)
                .zip(g2.by_ref().take(ns))
                .map(|(r1, r2)| {
                    (
                        ((i as f64) + r1) / (nx as f64),
                        ((j as f64) + r2) / (ny as f64),
                    )
                })
                .map(|(u, v)| camera.get_ray(u, v));

            let mut col = Vec3::default();
            for ray in r_rays {
                col += color(&ray, &world, &mut g, 0);
            }

            col /= ns as f64;

            col = Vec3::new(col.r().sqrt(), col.g().sqrt(), col.b().sqrt());

            let (ir, ig, ib) = col.irgb(255.99_f64);
            writeln!(&mut file, "{} {} {}", ir, ig, ib).expect("Unable to write to file");
        }
    }
}
