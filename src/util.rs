extern crate cgmath;
use cgmath::Vector3;

pub struct Ray {
    A: Vector3<f32>,
    B: Vector3<f32>,
}

impl Ray {
    pub fn new(a: Vector3<f32>, b: Vector3<f32>) -> Ray {
        Ray { A: a, B: b }
    }
    pub fn point_at(&self, t: f32) {
        self.A + t * self.B;
    }
    pub fn direction(&self) -> Vector3<f32> {
        self.B
    }
    pub fn origin(&self) -> Vector3<f32> {
        self.A
    }
}