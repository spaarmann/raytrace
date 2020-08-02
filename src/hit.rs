use crate::ray::Ray;
use crate::vec3::Vec3;
use std::ops::Range;

pub struct Hit {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl Hit {
    pub fn new(point: Vec3, outward_normal: Vec3, t: f64, ray_direction: Vec3) -> Self {
        let front_face = ray_direction.dot(outward_normal) < 0.0;
        Hit {
            point,
            normal: if front_face {
                outward_normal
            } else {
                -outward_normal
            },
            t,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: Ray, t_range: Range<f64>) -> Option<Hit>;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_range: Range<f64>) -> Option<Hit> {
        let oc = ray.origin - self.center;
        let a = ray.direction.mag_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.mag_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();

            let t = (-half_b - root) / a;
            if t_range.contains(&t) {
                let point = ray.at(t);
                return Some(Hit::new(
                    point,
                    (point - self.center) / self.radius,
                    t,
                    ray.direction,
                ));
            }

            let t = (-half_b + root) / a;
            if t_range.contains(&t) {
                let point = ray.at(t);
                return Some(Hit::new(
                    point,
                    (point - self.center) / self.radius,
                    t,
                    ray.direction,
                ));
            }
        }

        None
    }
}

pub struct HittableList<'a> {
    pub hittables: Vec<&'a dyn Hittable>,
}

impl<'a> Hittable for HittableList<'a> {
    fn hit(&self, ray: Ray, mut t_range: Range<f64>) -> Option<Hit> {
        let mut current_hit = None;

        for hittable in self.hittables.iter() {
            if let Some(new_hit) = hittable.hit(ray, t_range.clone()) {
                t_range.end = new_hit.t;
                current_hit = Some(new_hit)
            }
        }

        current_hit
    }
}
