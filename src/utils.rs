use oorandom::Rand32;
use std::ops::Range;
use ultraviolet::Vec3;

pub fn rand_unit_vec(rng: &mut Rand32) -> Vec3 {
    let a = rand_in_range(rng, 0.0..2.0 * std::f32::consts::PI);
    let z = rand_in_range(rng, -1.0..1.0);
    let r = (1.0 - z * z).sqrt();

    Vec3::new(r * a.cos(), r * a.sin(), z)
}

fn rand_in_range(rng: &mut Rand32, range: Range<f32>) -> f32 {
    range.start + (range.end - range.start) * rng.rand_float()
}
