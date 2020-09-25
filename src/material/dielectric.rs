use crate::object::HitRecord;
use crate::ray::Ray;
use crate::rgb::Rgb;
use rand::Rng;
use ultraviolet::Vec3;

pub struct Dielectric {
    pub refractive_idx: f32,
}

impl Dielectric {
    pub(super) fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        rng: &mut impl Rng,
    ) -> (Rgb, Ray) {
        let etai_over_etat = if hit_record.front_face {
            1.0 / self.refractive_idx
        } else {
            self.refractive_idx
        };

        let unit_direction = ray.direction.normalized();

        let cos_theta = -unit_direction.dot(hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let must_reflect = etai_over_etat * sin_theta > 1.0;
        let mut is_randomly_reflecting = || {
            let reflection_probability = schlick(cos_theta, etai_over_etat);
            rng.gen::<f32>() < reflection_probability
        };

        let direction = if must_reflect || is_randomly_reflecting() {
            unit_direction.reflected(hit_record.normal)
        } else {
            refract(unit_direction, hit_record.normal, etai_over_etat)
        };

        (
            Rgb::new(1.0, 1.0, 1.0),
            Ray {
                origin: hit_record.point,
                direction,
            },
        )
    }
}

fn refract(uv: Vec3, normal: Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = -uv.dot(normal);
    let r_out_perp = etai_over_etat * (uv + cos_theta * normal);
    let r_out_parallel = (1.0 - r_out_perp.mag_sq()).abs().sqrt() * -normal;

    r_out_perp + r_out_parallel
}

fn schlick(cosine: f32, refractive_idx: f32) -> f32 {
    let r0 = ((1.0 - refractive_idx) / (1.0 + refractive_idx)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
