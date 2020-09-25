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
    pub front_face: bool,
}

impl<'m> HitRecord<'m> {
    pub fn new(
        ray: &Ray,
        point: Vec3,
        outward_normal: Vec3,
        t: f32,
        material: &'m Material,
    ) -> Self {
        let front_face = ray.direction.dot(outward_normal) < 0.0;

        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Self {
            point,
            normal,
            t,
            material,
            front_face,
        }
    }
}
