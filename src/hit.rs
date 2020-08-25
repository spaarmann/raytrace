use crate::Material;
use crate::Ray;
use crate::Vec3;
use serde::{Deserialize, Serialize};
use std::ops::Range;

pub struct Hit<'a> {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: &'a dyn Material,
}

impl<'a> Hit<'a> {
    pub fn new(
        point: Vec3,
        outward_normal: Vec3,
        t: f64,
        ray_direction: Vec3,
        material: &'a dyn Material,
    ) -> Self {
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
            material,
        }
    }
}

#[typetag::serde]
pub trait Hittable: Sync {
    fn hit(&self, ray: Ray, t_range: Range<f64>) -> Option<Hit>;
}

#[derive(Serialize, Deserialize)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Box<dyn Material>,
}

#[typetag::serde]
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
                    self.material.as_ref(),
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
                    self.material.as_ref(),
                ));
            }
        }

        None
    }
}

#[derive(Serialize, Deserialize)]
pub struct HittableList {
    pub hittables: Vec<Box<dyn Hittable>>,
}

#[typetag::serde]
impl Hittable for HittableList {
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
