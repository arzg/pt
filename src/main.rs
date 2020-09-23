use image::png::PngEncoder;
use image::ColorType;
use pt::ray::Ray;
use pt::rgb::Rgb;
use std::fs::File;
use ultraviolet::Vec3;

fn main() -> anyhow::Result<()> {
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u16 = 400;
    let image_height = (f32::from(image_width) / aspect_ratio) as u16;

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::default();
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    let mut pixels = Vec::with_capacity(image_width as usize * image_height as usize);

    for y in (0..image_height).rev() {
        for x in 0..image_width {
            let u = f32::from(x) / (f32::from(image_width) - 1.0);
            let v = f32::from(y) / (f32::from(image_height) - 1.0);

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
        image_width.into(),
        image_height.into(),
        ColorType::Rgb8,
    )?;

    Ok(())
}

fn ray_color(ray: Ray) -> Rgb {
    let unit_direction = ray.direction.normalized();
    let t = 0.5 * (unit_direction.y + 1.0);

    Rgb::new(1.0, 1.0, 1.0) * (1.0 - t) + Rgb::new(0.5, 0.7, 1.0) * t
}
