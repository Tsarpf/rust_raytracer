extern crate cgmath;
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;

use cgmath::Vector3;
use cgmath::InnerSpace;

mod util;

fn main() {
    println!("Hello, world!");
    let mut file = BufWriter::new(File::create("test.ppm").unwrap());
    print_file(&mut file);
}

fn print_file(file: &mut BufWriter<File>) {
    let nx = 200;
    let ny = 100;
    write!(file, "P3\n{} {}\n255\n", nx, ny);

    let lower_left_corner = Vector3::new(-2.0, -1.0, -1.0);
    let horizontal = Vector3::new(4.0, 0.0, 0.0);
    let vertical = Vector3::new(0.0, 2.0, 0.0);
    let origin = Vector3::new(0.0, 0.0, 0.0);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u: f32 = (i as f32) / (nx as f32);
            let v: f32 = (j as f32) / (ny as f32);
            let ray = util::Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
            let col = color(&ray);

            let ir = (255.99 * col.x) as i32;
            let ig = (255.99 * col.y) as i32;
            let ib = (255.99 * col.z) as i32;
            write!(file, "{} {} {}\n", ir, ig, ib).unwrap();
        }
    }
}

fn color(ray: &util::Ray) -> Vector3<f32> {
    let mut t: f32 = hit_sphere(&Vector3::<f32>::new(0.0, 0.0, -1.0), 0.5, ray);
    if t > 0.0 {
        let n = (ray.point_at(t) - Vector3::new(0.0, 0.0, -1.)).normalize();
        return 0.5 * Vector3::<f32>::new(n.x + 1.0, n.y + 1.0, n.z + 1.0);
    }
    t = 0.5 * (ray.direction().normalize().y + 1.0);
    (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
}

// The basic function of a sphere centered at origin:
// x*x + y*y + z*z = R*R (radius)

// Spehere centered at (cx, cy, cz):
// (x-cx)*(x-cx) + (y-cy)*(y-cy) + (z-cz)*(z-cz) = R*R

// This is the same as
// dot((p-c), (p-c)) = R*R where p is a position vector we're comparing and
// c is the position vector of the center of the sphere.

// making the position vector a function of of t we get:
// t*t*dot(B,B) + 2*t*dot(B,A-C) + dot(A-C,A-C) - R*R = 0
// which is a basic quadratic function with 0/1/2 roots
// The roots are where the ray hits the sphere
fn hit_sphere(center: &Vector3<f32>, radius: f32, ray: &util::Ray) -> f32 {
    // center to ray origin
    let oc: Vector3<f32> = ray.origin() - center;
    let a = ray.direction().dot(ray.direction());
    let b = 2.0 * oc.dot(ray.direction());
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}