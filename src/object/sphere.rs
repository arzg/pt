use super::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use std::ops::Range;
use ultraviolet::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    pub(super) fn hit(&self, ray: &Ray, t_range: Range<f32>) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.mag_sq();
        let half_b = oc.dot(ray.direction);
        let c = oc.mag_sq() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();

            let t = (-half_b - root) / a;
            if t_range.contains(&t) {
                let point = ray.at(t);
                return Some(HitRecord::new(
                    ray,
                    point,
                    (point - self.center) / self.radius,
                    t,
                    &self.material,
                ));
            }

            let t = (-half_b + root) / a;
            if t_range.contains(&t) {
                let point = ray.at(t);
                return Some(HitRecord::new(
                    ray,
                    point,
                    (point - self.center) / self.radius,
                    t,
                    &self.material,
                ));
            }
        }

        None
    }
}
