use oorandom::Rand32;
use std::ops::Range;
use ultraviolet::Vec3;

fn rand_in_range(rng: &mut Rand32, range: Range<f32>) -> f32 {
    range.start + (range.end - range.start) * rng.rand_float()
}

pub fn rand_in_unit_sphere(rng: &mut Rand32) -> Vec3 {
    loop {
        let p = Vec3::new(
            rand_in_range(rng, -1.0..1.0),
            rand_in_range(rng, -1.0..1.0),
            rand_in_range(rng, -1.0..1.0),
        );

        if p.mag_sq() < 1.0 {
            return p;
        }
    }
}
