extern crate cgmath;
use cgmath::Vector3;
use cgmath::InnerSpace;
type Vec3 = Vector3<f32>;

pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(a: Vec3, b: Vec3) -> Ray {
        Ray {
            origin: a,
            direction: b,
        }
    }
    pub fn point_at(&self, t: &f32) -> Vec3 {
        self.origin + (*t) * self.direction
    }
    pub fn direction(&self) -> Vec3 {
        self.direction
    }
    pub fn origin(&self) -> Vec3 {
        self.origin
    }
}

pub struct Sphere {
    pub radius: f32,
    pub center: Vec3,
}

pub struct Hit {
    t: f32,
    pub p: Vec3,
    pub normal: Vec3,
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;
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
impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        // center to ray origin
        let oc: Vec3 = ray.origin() - self.center;
        let a = ray.direction().dot(ray.direction());
        let b = 2.0 * oc.dot(ray.direction());
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant > 0.0 {
            let plus: f32 = (-b + (b * b - 4.0 * a * c).sqrt()) / (2.0 * a);
            let minus: f32 = (-b - (b * b - 4.0 * a * c).sqrt()) / (2.0 * a);
            for temp in &[minus, plus] {
                if temp < &t_max && temp > &t_min {
                    let point = ray.point_at(temp);
                    return Some(Hit {
                        t: *temp,
                        p: point,
                        normal: (point - self.center) / self.radius,
                    });
                }
            }
        }
        return None;
    }
}

pub struct HitableList {
    pub list: Vec<Box<Hitable>>,
}

impl<'a> Hitable for &'a HitableList {
    // Would be cool to do this with a map and a filter
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let mut closest_so_far = t_max;
        let mut result: Option<Hit> = None;
        for obj in &self.list {
            match obj.hit(&ray, t_min, closest_so_far) {
                Some(hit) => {
                    closest_so_far = hit.t;
                    result = Some(hit);
                }
                None => {}
            }
        }
        result
    }
}
