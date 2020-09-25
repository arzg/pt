use crate::object::HitRecord;
use crate::ray::Ray;
use crate::rgb::Rgb;
use crate::utils;
use rand::Rng;

pub struct Lambertian {
    pub albedo: Rgb,
}

impl Lambertian {
    pub(super) fn scatter(&self, hit_record: &HitRecord, rng: &mut impl Rng) -> (Rgb, Ray) {
        let scatter_direction = hit_record.normal + utils::rand_unit_vec(rng);
        let scattered = Ray {
            origin: hit_record.point,
            direction: scatter_direction,
        };

        (self.albedo, scattered)
    }
}
