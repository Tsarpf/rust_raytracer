extern crate cgmath;
use cgmath::Vector3;

pub struct Ray {
    origin: Vector3<f32>,
    direction: Vector3<f32>,
}

impl Ray {
    pub fn new(a: Vector3<f32>, b: Vector3<f32>) -> Ray {
        Ray {
            origin: a,
            direction: b,
        }
    }
    pub fn point_at(&self, t: f32) -> Vector3<f32> {
        self.origin + t * self.direction
    }
    pub fn direction(&self) -> Vector3<f32> {
        self.direction
    }
    pub fn origin(&self) -> Vector3<f32> {
        self.origin
    }
}