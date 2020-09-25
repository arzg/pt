use pt::camera::Camera;
use pt::material::{
    Dielectric, DiffuseLight, HandleResult, Lambertian, Material, Metal, Scattered,
};
use pt::object::{hit_iter, Object, Sphere};
use pt::ray::Ray;
use pt::rgb::Rgb;
use rand::Rng;
use rayon::prelude::*;
use std::fs::File;
use ultraviolet::Vec3;

const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_WIDTH: u16 = 1280;
const IMAGE_HEIGHT: u16 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u16;

const SAMPLES_PER_PIXEL: u16 = 100;
const MAX_DEPTH: u16 = 50;

fn main() -> anyhow::Result<()> {
    let camera = {
        let look_from = Vec3::new(0.0, 0.0, 4.0);
        let look_at = Vec3::new(0.0, 0.0, -1.0);
        let focus_distance = (look_from - look_at).mag();

        Camera::new(
            look_from,
            look_at,
            Vec3::new(0.0, 1.0, 0.0),
            30.0,
            ASPECT_RATIO,
            1.0,
            focus_distance,
        )
    };

    let world = [
        Object::Sphere(Sphere {
            center: Vec3::new(0.0, -100.5, -1.0),
            radius: 100.0,
            material: Material::Lambertian(Lambertian {
                albedo: Rgb::new(0.8, 0.8, 0.9),
            }),
        }),
        Object::Sphere(Sphere {
            center: Vec3::new(0.0, 0.0, -1.0),
            radius: 0.5,
            material: Material::Lambertian(Lambertian {
                albedo: Rgb::new(0.7, 0.3, 0.3),
            }),
        }),
        Object::Sphere(Sphere {
            center: Vec3::new(-1.0, 0.0, -1.0),
            radius: 0.5,
            material: Material::Metal(Metal {
                albedo: Rgb::new(0.8, 0.8, 0.8),
                fuzz: 0.3,
            }),
        }),
        Object::Sphere(Sphere {
            center: Vec3::new(1.0, 0.0, -1.0),
            radius: 0.5,
            material: Material::Dielectric(Dielectric {
                refractive_idx: 1.5,
            }),
        }),
        Object::Sphere(Sphere {
            center: Vec3::new(0.0, 0.8, -1.0),
            radius: 0.1,
            material: Material::DiffuseLight(DiffuseLight {
                emission: Rgb::new(100.0, 100.0, 100.0),
            }),
        }),
        Object::Sphere(Sphere {
            center: Vec3::new(1.6, 0.3, -1.0),
            radius: 0.1,
            material: Material::DiffuseLight(DiffuseLight {
                emission: Rgb::new(100.0, 100.0, 100.0),
            }),
        }),
        Object::Sphere(Sphere {
            center: Vec3::new(-4.0, 1.0, 2.0),
            radius: 0.5,
            material: Material::DiffuseLight(DiffuseLight {
                emission: Rgb::new(60.0, 60.0, 60.0),
            }),
        }),
    ];

    let image_coords = (0..IMAGE_HEIGHT)
        .into_par_iter()
        .rev()
        .flat_map(|y| (0..IMAGE_WIDTH).into_par_iter().map(move |x| (x, y)));

    let pixels: Vec<_> = image_coords
        .map(|(x, y)| {
            let mut rng = rand::thread_rng();

            let mut pixel_color = Rgb::new(0.0, 0.0, 0.0);

            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (f32::from(x) + rng.gen::<f32>()) / (f32::from(IMAGE_WIDTH) - 1.0);
                let v = (f32::from(y) + rng.gen::<f32>()) / (f32::from(IMAGE_HEIGHT) - 1.0);
                let ray = camera.get_ray(u, v, &mut rng);

                pixel_color +=
                    ray_color(&world, &ray, Rgb::new(0.0, 0.0, 0.0), &mut rng, MAX_DEPTH);
            }

            pixel_color
        })
        .collect();

    let u8s: Vec<_> = pixels
        .iter()
        .flat_map(|rgb| rgb.iter(SAMPLES_PER_PIXEL))
        .collect();

    write_image("image.png", &u8s)?;

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

fn ray_color(world: &[Object], ray: &Ray, background: Rgb, rng: &mut impl Rng, depth: u16) -> Rgb {
    if depth == 0 {
        return Rgb::new(0.0, 0.0, 0.0);
    }

    if let Some(hit_record) = hit_iter(world.iter(), ray, 0.0001..f32::MAX) {
        match hit_record.material.handle_ray(ray, &hit_record, rng) {
            HandleResult::Scattered(Some(Scattered {
                attenuation,
                ray: scattered,
            })) => attenuation * ray_color(world, &scattered, background, rng, depth - 1),
            HandleResult::Scattered(None) => background,
            HandleResult::Emitted { color } => color,
        }
    } else {
        background
}
}
