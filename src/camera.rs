use crate::Ray;
use crate::Vec3;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(
        origin: Vec3,
        up: Vec3,
        forward: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        focal_length: f64,
    ) -> Self {
        let theta = vfov * std::f64::consts::PI / 180.0;
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let up = up.normalized();
        let forward = forward.normalized();

        let right = up.cross(forward);

        let horizontal = viewport_width * right;
        let vertical = viewport_height * up;
        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin - horizontal * 0.5 - vertical * 0.5 + forward * focal_length,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
