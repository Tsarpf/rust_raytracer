extern crate cgmath;
extern crate rand;
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;

use cgmath::Vector3;
use cgmath::InnerSpace;
type Vec3 = Vector3<f32>;

mod util;
mod camera;
use camera::Camera;

fn main() {
    println!("Hello, world!");
    let mut file = BufWriter::new(File::create("test.ppm").unwrap());
    print_file(&mut file);
}

fn print_file(file: &mut BufWriter<File>) {
    let nx = 200;
    let ny = 100;
    let ns = 100;
    let _ = write!(file, "P3\n{} {}\n255\n", nx, ny);

    let camera = Camera::new();

    let world = util::HitableList {
        list: vec![Box::new(util::Sphere {
                       center: Vec3::new(0.0, 0.0, -1.0),
                       radius: 0.5,
                   }),
                   Box::new(util::Sphere {
                       center: Vec3::new(0.0, -100.5, -1.0),
                       radius: 100.0,
                   })],
    };

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0., 0., 0.);
            for _ in 0..ns {
                let u: f32 = (i as f32 + rand::random::<f32>()) / (nx as f32);
                let v: f32 = (j as f32 + rand::random::<f32>()) / (ny as f32);
                let ray = camera.get_ray(u, v);
                col += color(&ray, &world);
            }
            col /= ns as f32;
            let ir = (255.99 * col.x) as i32;
            let ig = (255.99 * col.y) as i32;
            let ib = (255.99 * col.z) as i32;
            write!(file, "{} {} {}\n", ir, ig, ib).unwrap();
        }
    }
}

fn color<T: util::Hitable>(ray: &util::Ray, world: T) -> Vec3 {
    match world.hit(ray, 0.0, std::f32::MAX) {
        Some(hit) => 0.5 * Vec3::new(hit.normal.x + 1.0, hit.normal.y + 1.0, hit.normal.z + 1.0),
        None => {
            let t = 0.5 * (ray.direction().normalize().y + 1.0);
            (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
        }
    }
}