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
    write!(file, "P3\n{} {}\n255", nx, ny);
}