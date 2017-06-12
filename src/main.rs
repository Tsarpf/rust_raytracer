extern crate cgmath;
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;

fn main() {
    println!("Hello, world!");
    let mut file = BufWriter::new(File::create("test.ppm").unwrap());
    print_file(&mut file);
}

fn print_file(file: &mut BufWriter<File>) {
    let nx = 200;
    let ny = 100;
    write!(file, "P3\n{} {}\n255\n", nx, ny);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let rgb = cgmath::Vector3::new(
                (i as f32)/(nx as f32),
                (j as f32)/(ny as f32),
                0.2);
            let ir = (255.99 * rgb.x) as i32;
            let ig = (255.99 * rgb.y) as i32;
            let ib = (255.99 * rgb.z) as i32;
            write!(file, "{} {} {}\n", ir, ig, ib).unwrap();
        }
    }
}