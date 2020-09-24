mod sphere;
pub use sphere::Sphere;

use crate::material::Material;
use crate::ray::Ray;
use std::ops::Range;
use ultraviolet::Vec3;

pub enum Object {
    Sphere(Sphere),
}

impl Object {
    fn hit(&self, ray: &Ray, t_range: Range<f32>) -> Option<HitRecord> {
        match self {
            Self::Sphere(sphere) => sphere.hit(ray, t_range),
        }
    }
}

pub fn hit_iter<'o>(
    objects: impl Iterator<Item = &'o Object>,
    ray: &Ray,
    t_range: Range<f32>,
) -> Option<HitRecord<'o>> {
    let mut hit_record = None;
    let mut closest_so_far = t_range.end;

    for object in objects {
        if let Some(rec) = object.hit(ray, t_range.start..closest_so_far) {
            closest_so_far = rec.t;
            hit_record = Some(rec);
        }
    }

    hit_record
}

pub struct HitRecord<'m> {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub material: &'m Material,
}
