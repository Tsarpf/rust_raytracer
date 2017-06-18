extern crate cgmath;
extern crate rand;
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;

use cgmath::Vector3;
use cgmath::InnerSpace;
type Vec3 = Vector3<f32>;

mod materials;
use materials::Lambertian;
use materials::Metal;
use materials::Dielectric;
mod util;
use util::Hitable;
use util::HitableList;
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

    let lookfrom = Vec3::new(3., 3., 2.);
    let lookat = Vec3::new(0., 0., -1.);
    let dist_to_focus = (lookfrom-lookat).magnitude();
    let aperture = 2.0;

    let camera = Camera::new(lookfrom, lookat, Vec3::new(0., 1., 0.), 20., nx as f32 / ny as f32, aperture, dist_to_focus);

    let mat1 = Lambertian {
        albedo: Vec3::new(0.1, 0.2, 0.5)
    };
    let sphere1 = util::Sphere {
        center: Vec3::new(0.0, 0., -1.0),
        radius: 0.5,
        material: &mat1
    };
    
    let mat2 = Lambertian {
        albedo: Vec3::new(0.8, 0.8, 0.0),
    };
    let sphere2 = util::Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: &mat2
    };

    let mat3 = Metal {
        albedo: Vec3::new(0.8, 0.6, 0.2),
        fuzz: 1.0
    };
    let sphere3 = util::Sphere {
        center: Vec3::new(1.0, 0., -1.0),
        radius: 0.5,
        material: &mat3
    };

    let mat4 = Dielectric {
        ref_idx: 1.5
    };
    let sphere4 = util::Sphere {
        center: Vec3::new(-1.0, 0., -1.0),
        radius: 0.5,
        material: &mat4
    };

    let mat5 = Dielectric {
        ref_idx: 1.5
    };
    let sphere5 = util::Sphere {
        center: Vec3::new(-1.0, 0., -1.0),
        radius: -0.45,
        material: &mat5
    };

    let world = HitableList {
        list: vec![&sphere1, &sphere2, &sphere3, &sphere4, &sphere5],
    };
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0., 0., 0.);
            for _ in 0..ns {
                let u: f32 = (i as f32 + rand_f32()) / (nx as f32);
                let v: f32 = (j as f32 + rand_f32()) / (ny as f32);
                let ray = camera.get_ray(u, v);
                col += color(&ray, &world, 0);
            }
            col /= ns as f32;
            col = Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());
            let ir = (255.99 * col.x) as i32;
            let ig = (255.99 * col.y) as i32;
            let ib = (255.99 * col.z) as i32;
            write!(file, "{} {} {}\n", ir, ig, ib).unwrap();
        }
    }
}

fn color(ray: &util::Ray, world: &util::HitableList, depth: i32) -> Vec3 {
    match world.hit(ray, 0.001, std::f32::MAX) {
        Some(hit) => {
            match hit.material.scatter(ray, &hit) {
                Some((attenuation, scattered)) => {
                    if depth < 50 {
                        let col = color(&scattered, &world, depth+1);
                        Vec3::new(attenuation.x * col.x, attenuation.y * col.y, attenuation.z * col.z)
                    } else {
                        Vec3::new(0.,0.,0.)
                    }
                },
                None =>  Vec3::new(0.,0.,0.)
            }
        },
        None => {
            let t = 0.5 * (ray.direction().normalize().y + 1.0);
            (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
        }
    }
}

fn rand_f32() -> f32 {
    rand::random::<f32>()
}

fn random_in_unit_sphere() -> Vec3 {
    let mut p: Vec3;
    loop {
        p = 2.0 * Vec3::new(rand_f32(), rand_f32(), rand_f32()) - Vec3::new(1.,1.,1.);
        if p.magnitude2() <= 1.0 { break; }
    }
    p
}