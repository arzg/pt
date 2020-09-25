use rand::Rng;
use ultraviolet::Vec3;

pub(crate) fn rand_unit_vec(rng: &mut impl Rng) -> Vec3 {
    let a = rng.gen_range(0.0, 2.0 * std::f32::consts::PI);
    let z: f32 = rng.gen_range(-1.0, 1.0);
    let r = (1.0 - z * z).sqrt();

    Vec3::new(r * a.cos(), r * a.sin(), z)
}

pub(crate) fn rand_in_unit_sphere(rng: &mut impl Rng) -> Vec3 {
    loop {
        let p = Vec3::new(
            rng.gen_range(-1.0, 1.0),
            rng.gen_range(-1.0, 1.0),
            rng.gen_range(-1.0, 1.0),
        );

        if p.mag_sq() < 1.0 {
            return p;
        }
    }
}
