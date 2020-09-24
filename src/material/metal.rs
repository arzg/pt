use crate::object::HitRecord;
use crate::ray::Ray;
use crate::rgb::Rgb;

pub struct Metal {
    pub albedo: Rgb,
}

impl Metal {
    pub(super) fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Rgb, Ray)> {
        let reflected = ray.direction.normalized().reflected(hit_record.normal);
        let scattered = Ray {
            origin: hit_record.point,
            direction: reflected,
        };

        if scattered.direction.dot(hit_record.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
