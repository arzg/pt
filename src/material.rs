mod lambertian;
mod metal;
pub use lambertian::Lambertian;
pub use metal::Metal;

use crate::object::HitRecord;
use crate::ray::Ray;
use crate::rgb::Rgb;
use oorandom::Rand32;

pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
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
            Self::Metal(metal) => metal.scatter(ray, hit_record),
        }
    }
}
