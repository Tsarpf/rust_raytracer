use cgmath::Vector3;
use cgmath::InnerSpace;
type Vec3 = Vector3<f32>;

use super::util::Ray;
use super::util::Hit;
use super::random_in_unit_sphere;
use super::rand_f32;

pub struct Lambertian {
    pub albedo: Vec3
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<(Vec3, Ray)> {
        let target = hit.p + hit.normal + super::random_in_unit_sphere();
        let scattered = Ray::new(hit.p,target - hit.p);
        Some((self.albedo, scattered))
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<(Vec3, Ray)> {
        let reflected = reflect(ray_in.direction().normalize(), hit.normal);
        let scattered = Ray::new(hit.p, reflected + self.fuzz * random_in_unit_sphere());
        match scattered.direction().dot(hit.normal) > 0.0 {
            true => Some((self.albedo, scattered)),
            false => None
        }
    }
}

pub struct Dielectric {
    pub ref_idx: f32
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<(Vec3, Ray)> {
        let outward_normal: Vec3;
        let reflected = reflect(ray_in.direction(), hit.normal);
        let ni_over_nt: f32;
        let attenuation = Vec3::new(1., 1., 1.);
        let cosine: f32;
        let reflect_prob: f32;
        if ray_in.direction().dot(hit.normal) > 0. {
            outward_normal = -hit.normal;
            ni_over_nt = self.ref_idx;
            cosine = self.ref_idx * ray_in.direction().dot(hit.normal) / ray_in.direction().magnitude();
        } else {
            outward_normal = hit.normal;
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = -ray_in.direction().dot(hit.normal) / ray_in.direction().magnitude();
        }
        match refract(ray_in.direction(), outward_normal, ni_over_nt) {
            Some(refracted) => {
                reflect_prob = schlick(cosine, self.ref_idx);
                if rand_f32() < reflect_prob {
                    Some((attenuation, Ray::new(hit.p, reflected)))
                } else {
                    Some((attenuation, Ray::new(hit.p, refracted)))
                }
            },
            None => {
                Some((attenuation, Ray::new(hit.p, reflected)))
            }
        }
    }
}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<(Vec3, Ray)>;
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * (v.dot(n)) * n
}

fn refract(v: Vec3, n: Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = v.normalize();
    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt*(1.0-dt*dt);
    if discriminant > 0. {
        Some(ni_over_nt * (uv - n * dt) - n * discriminant.sqrt())
    } else {
        None
    }
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = ((1. - ref_idx) / (1. + ref_idx)).powi(2);
    r0 + (1. - r0) * (1. - cosine).powi(5) 
}