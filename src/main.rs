use oorandom::Rand32;
use pt::camera::Camera;
use pt::object::{hit_iter, Object, Sphere};
use pt::ray::Ray;
use pt::rgb::Rgb;
use pt::utils::rand_in_unit_sphere;
use std::fs::File;
use std::ops::RangeInclusive;
use ultraviolet::Vec3;

const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_WIDTH: u16 = 400;
const IMAGE_HEIGHT: u16 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u16;

const SAMPLES_PER_PIXEL: u16 = 80;
const MAX_DEPTH: u16 = 50;

fn main() -> anyhow::Result<()> {
    let camera = Camera::new(ASPECT_RATIO);

    let world = [
        Object::Sphere(Sphere {
            center: Vec3::new(0.0, 0.0, -1.0),
            radius: 0.5,
        }),
        Object::Sphere(Sphere {
            center: Vec3::new(0.0, -100.5, -1.0),
            radius: 100.0,
        }),
    ];

    let mut rng = Rand32::new(100);

    let image_coords = (0..IMAGE_HEIGHT)
        .rev()
        .flat_map(|y| (0..IMAGE_WIDTH).map(move |x| (x, y)));

    let pixels: Vec<_> = image_coords
        .flat_map(|(x, y)| {
            let mut pixel_color = Rgb::new(0.0, 0.0, 0.0);

            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (f32::from(x) + rng.rand_float()) / (f32::from(IMAGE_WIDTH) - 1.0);
                let v = (f32::from(y) + rng.rand_float()) / (f32::from(IMAGE_HEIGHT) - 1.0);
                let ray = camera.get_ray(u, v);

                pixel_color += ray_color(&world, &ray, &mut rng, MAX_DEPTH);
            }

            pixel_color.iter(SAMPLES_PER_PIXEL)
        })
        .collect();

    write_image("image.png", &pixels)?;

    Ok(())
}

fn write_image(path: &str, pixels: &[u8]) -> anyhow::Result<()> {
    let file = File::create(path)?;
    let mut png_encoder = png::Encoder::new(file, IMAGE_WIDTH.into(), IMAGE_HEIGHT.into());
    png_encoder.set_color(png::ColorType::RGB);
    png_encoder.set_depth(png::BitDepth::Eight);

    let mut png_writer = png_encoder.write_header()?;
    png_writer.write_image_data(&pixels)?;

    Ok(())
}

fn ray_color(world: &[Object], ray: &Ray, rng: &mut Rand32, depth: u16) -> Rgb {
    if depth == 0 {
        return Rgb::new(0.0, 0.0, 0.0);
    }

    if let Some(hit_record) = hit_iter(world.iter(), ray, 0.0001..f32::MAX) {
        let target = hit_record.point + hit_record.normal + rand_in_unit_sphere(rng);

        return ray_color(
            world,
            &Ray {
                origin: hit_record.point,
                direction: target - hit_record.point,
            },
            rng,
            depth - 1,
        ) * 0.5;
    }

    let unit_direction = ray.direction.normalized();
    let t = scale_to_between_zero_and_one(unit_direction.y, -1.0..=1.0);

    linearly_interpolate(t, Rgb::new(1.0, 1.0, 1.0), Rgb::new(0.5, 0.7, 1.0))
}

fn linearly_interpolate(t: f32, at_zero_i_want: Rgb, at_one_i_want: Rgb) -> Rgb {
    at_zero_i_want * (1.0 - t) + at_one_i_want * t
}

fn scale_to_between_zero_and_one(val: f32, range: RangeInclusive<f32>) -> f32 {
    debug_assert!(range.contains(&val));

    let (val_with_min_at_zero, max_accounting_for_min_at_zero) = if range.start() < &0.0 {
        (val - range.start(), range.end() - range.start())
    } else {
        (val + range.start(), range.end() + range.start())
    };

    val_with_min_at_zero * (1.0 / max_accounting_for_min_at_zero)
}
