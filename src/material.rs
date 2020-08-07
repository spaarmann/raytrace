use crate::Hit;
use crate::Ray;
use crate::Vec3;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<(Vec3, Ray)>;
}

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit: &Hit) -> Option<(Vec3, Ray)> {
        let scatter_direction = hit.normal + Vec3::random_unit_vector();
        let scattered = Ray::new(hit.point, scatter_direction);
        Some((self.albedo, scattered))
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

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
