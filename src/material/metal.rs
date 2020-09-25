use super::Scattered;
use crate::object::HitRecord;
use crate::ray::Ray;
use crate::rgb::Rgb;
use crate::utils;
use rand::Rng;

pub struct Metal {
    pub albedo: Rgb,
    pub fuzz: f32,
}

impl Metal {
    pub(super) fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        rng: &mut impl Rng,
    ) -> Option<Scattered> {
        let reflected = ray.direction.normalized().reflected(hit_record.normal);
        let scattered = Ray {
            origin: hit_record.point,
            direction: reflected + utils::rand_in_unit_sphere(rng) * self.fuzz,
        };

        if scattered.direction.dot(hit_record.normal) > 0.0 {
            Some(Scattered {
                attenuation: self.albedo,
                ray: scattered,
            })
        } else {
            None
        }
    }
}
