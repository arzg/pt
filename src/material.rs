mod lambertian;
pub use lambertian::Lambertian;

use crate::object::HitRecord;
use crate::ray::Ray;
use crate::rgb::Rgb;
use oorandom::Rand32;

pub enum Material {
    Lambertian(Lambertian),
}

impl Material {
    pub fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        rng: &mut Rand32,
    ) -> Option<(Rgb, Ray)> {
        match self {
            Self::Lambertian(lambertian) => Some(lambertian.scatter(hit_record, rng)),
        }
    }
}
