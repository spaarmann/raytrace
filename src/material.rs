use crate::Hit;
use crate::Ray;
use crate::Vec3;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[typetag::serde]
pub trait Material: Sync {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<(Vec3, Ray)>;
}

#[derive(Serialize, Deserialize)]
pub struct Lambertian {
    pub albedo: Vec3,
}

#[typetag::serde]
impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit: &Hit) -> Option<(Vec3, Ray)> {
        let scatter_direction = hit.normal + Vec3::random_unit_vector();
        let scattered = Ray::new(hit.point, scatter_direction);
        Some((self.albedo, scattered))
    }
}

#[derive(Serialize, Deserialize)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

#[typetag::serde]
impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<(Vec3, Ray)> {
        let reflected = ray_in.direction.normalized().reflect(hit.normal)
            + self.fuzz * Vec3::random_in_unit_sphere();
        if reflected.dot(hit.normal) > 0.0 {
            Some((self.albedo, Ray::new(hit.point, reflected)))
        } else {
            None
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Dielectric {
    pub refraction_index: f64,
}

#[typetag::serde]
impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<(Vec3, Ray)> {
        let etai_over_etat = if hit.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let normalized_direction = ray_in.direction.normalized();
        let cos_theta = (-normalized_direction).dot(hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        if etai_over_etat * sin_theta > 1.0 {
            return Some((
                Vec3::ONE,
                Ray::new(hit.point, normalized_direction.reflect(hit.normal)),
            ));
        }

        let mut rng = rand::thread_rng();
        let reflect_prob = self.schlick(cos_theta);
        if rng.gen::<f64>() < reflect_prob {
            return Some((
                Vec3::ONE,
                Ray::new(hit.point, normalized_direction.reflect(hit.normal)),
            ));
        }

        let refracted = normalized_direction.refract(hit.normal, etai_over_etat);
        Some((Vec3::ONE, Ray::new(hit.point, refracted)))
    }
}

impl Dielectric {
    fn schlick(&self, cosine: f64) -> f64 {
        let r0 = (1.0 - self.refraction_index) / (1.0 + self.refraction_index);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}
