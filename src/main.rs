use image::png::PngEncoder;
use image::ColorType;
use pt::ray::Ray;
use pt::rgb::Rgb;
use std::fs::File;
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

    let mut pixels = Vec::with_capacity(IMAGE_WIDTH as usize * IMAGE_HEIGHT as usize);

    for y in (0..IMAGE_HEIGHT).rev() {
        for x in 0..IMAGE_WIDTH {
            let u = f32::from(x) / (f32::from(IMAGE_WIDTH) - 1.0);
            let v = f32::from(y) / (f32::from(IMAGE_HEIGHT) - 1.0);

            let ray = Ray {
                origin,
                direction: lower_left_corner + u * horizontal + v * vertical - origin,
            };
            let pixel_color = ray_color(ray);

            pixels.push(pixel_color);
        }
    }

    let u8s: Vec<_> = pixels.iter().map(|rgb| rgb.into_iter()).flatten().collect();

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
    let t = 0.5 * (unit_direction.y + 1.0);

    Rgb::new(1.0, 1.0, 1.0) * (1.0 - t) + Rgb::new(0.5, 0.7, 1.0) * t
}
