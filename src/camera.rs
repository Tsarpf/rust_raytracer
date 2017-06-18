use cgmath::Vector3;
use util;
type Vec3 = Vector3<f32>;
use cgmath::InnerSpace;
use std::f32::consts::PI;
use super::rand_f32;

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
    lens_radius: f32,
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f32, aspect: f32, aperture: f32, focus_dist: f32) -> Camera {
        let theta = vfov * PI / 180.;
        let half_height = (theta/2.).tan();
        let half_width = aspect * half_height;
        let origin = lookfrom;
        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        Camera {
            lower_left_corner: origin - half_width*focus_dist*u - half_height*focus_dist*v - focus_dist*w,
            horizontal: 2. * half_width * focus_dist * u,
            vertical: 2. * half_height * focus_dist * v,
            origin: lookfrom,
            lens_radius: aperture / 2.,
            u: u,
            v: v,
            w: w,
        }
    }
    pub fn get_ray(&self, s: f32, t: f32) -> util::Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        util::Ray::new(self.origin + offset,
                       self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset)
    }
}

fn random_in_unit_disk() -> Vec3 {
    let mut p: Vec3;
    loop {
        p = 2.0 * Vec3::new(rand_f32(), rand_f32(), 0.) - Vec3::new(1.,1.,0.);
        if p.dot(p) < 1. { break; }
    }
    p
}