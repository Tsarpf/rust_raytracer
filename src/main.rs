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
    let nx = 400;
    let ny = 200;
    let ns = 10;
    let _ = write!(file, "P3\n{} {}\n255\n", nx, ny);

    let lookfrom = Vec3::new(10., 2.5, 7.);
    let lookat = Vec3::new(0., 0., 0.);
    let dist_to_focus = 10.0;
    let aperture = 0.01;

    let camera = Camera::new(lookfrom, lookat, Vec3::new(0., 1., 0.), 20., nx as f32 / ny as f32, aperture, dist_to_focus);

    let n = 500.0;
    let mat1 = Lambertian {
        albedo: Vec3::new(0.5, 0.5, 0.5)
    };
    let sphere1 = util::Sphere {
        center: Vec3::new(0.0, -100., 0.),
        radius: 0.5,
        material: Box::new(mat1)
    };

    let mut list: Vec<Box<Hitable>> = vec![Box::new(sphere1)];

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand_f32();
            let center = Vec3::new((a as f32) + 0.9 * rand_f32(), 0.2, (b as f32) + 0.9 * rand_f32());
            if (center - Vec3::new(4., 0.2, 0.)).magnitude() > 0.9 {
                if choose_mat < 0.8 { // diffuse
                    list.push(Box::new(util::Sphere {
                        center: center,
                        radius: 0.2,
                        material: Box::new(Lambertian {
                            albedo: Vector3::new(rand_f32() * rand_f32(), rand_f32() * rand_f32(), rand_f32() * rand_f32())
                        })
                    }));
                } else if choose_mat < 0.95 { // metal
                    list.push(Box::new(util::Sphere {
                        center: center,
                        radius: 0.2,
                        material: Box::new(Metal {
                            albedo: Vec3::new(0.5 * (1. + rand_f32()), 0.5 * (1. + rand_f32()), 0.5 * (1. + rand_f32())),
                            fuzz: 0.5 * rand_f32()
                        })
                    }));
                } else { // glass
                    list.push(Box::new(util::Sphere {
                        center: center,
                        radius: 0.2,
                        material: Box::new(Dielectric {
                            ref_idx: 1.5
                        })
                    }));
                }

            } 
        }
    }
    let mat2 = Dielectric {
        ref_idx: 1.5
    };
    let sphere2 = util::Sphere {
        center: Vec3::new(0., 1., 0.),
        radius: 1.0,
        material: Box::new(mat2)
    };

    let mat3 = Lambertian {
        albedo: Vec3::new(0.8, 0.2, 0.1)
    };
    let sphere3 = util::Sphere {
        center: Vec3::new(-4., 1., 0.),
        radius: 1.0,
        material: Box::new(mat3)
    };
    
    let mat4 = Metal {
        albedo: Vec3::new(0.7, 0.6, 0.5),
        fuzz: 0.0
    };
    let sphere4 = util::Sphere {
        center: Vec3::new(4., 1., 0.),
        radius: 1.0,
        material: Box::new(mat4)
    };


    list.push(Box::new(sphere2));
    list.push(Box::new(sphere3));
    list.push(Box::new(sphere4));


    //let world = HitableList {
    //    list: vec![Box::new(sphere4)],
    //};
    let world = HitableList {
        list: list,
    };
    for j in (0..ny).rev() {
        println!("on line number {}", j);
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