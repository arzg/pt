mod dielectric;
mod diffuse_light;
mod lambertian;
mod metal;
pub use dielectric::Dielectric;
pub use diffuse_light::DiffuseLight;
pub use lambertian::Lambertian;
pub use metal::Metal;

use crate::object::HitRecord;
use crate::ray::Ray;
use crate::rgb::Rgb;
use rand::Rng;

pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
    DiffuseLight(DiffuseLight),
}

impl Material {
    pub fn handle_ray(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        rng: &mut impl Rng,
    ) -> HandleResult {
        match self {
            Self::Lambertian(lambertian) => {
                HandleResult::Scattered(Some(lambertian.scatter(hit_record, rng)))
            }
            Self::Metal(metal) => HandleResult::Scattered(metal.scatter(ray, hit_record, rng)),
            Self::Dielectric(dielectric) => {
                HandleResult::Scattered(Some(dielectric.scatter(ray, hit_record, rng)))
            }
            Self::DiffuseLight(diffuse_light) => HandleResult::Emitted {
                color: diffuse_light.emission,
            },
        }
    }

    pub fn emit(&self) -> Option<Rgb> {
        if let Self::DiffuseLight(DiffuseLight { emission }) = self {
            Some(*emission)
        } else {
            None
        }
    }
}

pub enum HandleResult {
    Scattered(Option<Scattered>),
    Emitted { color: Rgb },
}

pub struct Scattered {
    pub attenuation: Rgb,
    pub ray: Ray,
}
