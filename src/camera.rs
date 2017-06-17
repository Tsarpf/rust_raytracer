use cgmath::Vector3;
use util;
type Vec3 = Vector3<f32>;

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            lower_left_corner: Vec3::new(-2.0, -1.0, -1.0),
            horizontal: Vec3::new(4.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.0, 0.0),
            origin: Vec3::new(0.0, 0.0, 0.0),
        }
    }
    pub fn get_ray(&self, u: f32, v: f32) -> util::Ray {
        util::Ray::new(self.origin,
                       self.lower_left_corner + u * self.horizontal + v * self.vertical)
    }
}