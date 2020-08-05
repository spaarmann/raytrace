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
