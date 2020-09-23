use image::png::PngEncoder;
use image::ColorType;
use pt::ray::Ray;
use pt::rgb::Rgb;
use std::fs::File;
use std::ops::Range;
use ultraviolet::Vec3;

const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_WIDTH: u16 = 400;
const IMAGE_HEIGHT: u16 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u16;

const VIEWPORT_HEIGHT: f32 = 2.0;
const VIEWPORT_WIDTH: f32 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f32 = 1.0;

fn main() -> anyhow::Result<()> {
    let origin = Vec3::default();
    let horizontal = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

    let rgbs = (0..IMAGE_HEIGHT)
        .rev()
        .flat_map(|y| (0..IMAGE_WIDTH).map(move |x| (x, y)))
        .map(|(x, y)| {
            let u = f32::from(x) / (f32::from(IMAGE_WIDTH) - 1.0);
            let v = f32::from(y) / (f32::from(IMAGE_HEIGHT) - 1.0);

            let ray = Ray {
                origin,
                direction: lower_left_corner + u * horizontal + v * vertical - origin,
            };

            ray_color(ray)
        });

    let u8s: Vec<_> = rgbs.map(|rgb| rgb.into_iter()).flatten().collect();

    let file = File::create("image.png")?;
    let png_encoder = PngEncoder::new(file);

    png_encoder.encode(
        &u8s,
        IMAGE_WIDTH.into(),
        IMAGE_HEIGHT.into(),
        ColorType::Rgb8,
    )?;

    Ok(())
}

fn ray_color(ray: Ray) -> Rgb {
    let unit_direction = ray.direction.normalized();
    let t = scale_to_between_zero_and_one(unit_direction.y, -1.0..1.0);

    linearly_interpolate(t, Rgb::new(1.0, 1.0, 1.0), Rgb::new(0.5, 0.7, 1.0))
}

fn linearly_interpolate(t: f32, at_zero_i_want: Rgb, at_one_i_want: Rgb) -> Rgb {
    at_zero_i_want * (1.0 - t) + at_one_i_want * t
}

fn scale_to_between_zero_and_one(val: f32, range: Range<f32>) -> f32 {
    debug_assert!(range.contains(&val));

    let (val_with_min_at_zero, max_accounting_for_min_at_zero) = if range.start < 0.0 {
        (val - range.start, range.end - range.start)
    } else {
        (val + range.start, range.end + range.start)
    };

    val_with_min_at_zero * (1.0 / max_accounting_for_min_at_zero)
}
