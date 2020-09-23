use image::png::PngEncoder;
use image::ColorType;
use pt::rgb::Rgb;
use std::fs::File;

fn main() -> anyhow::Result<()> {
    let rgbs = [Rgb {
        red: 0.0,
        green: 1.0,
        blue: 0.0,
    }];

    let u8s: Vec<_> = rgbs.iter().map(|rgb| rgb.into_iter()).flatten().collect();

    let file = File::create("image.png")?;
    let png_encoder = PngEncoder::new(file);

    png_encoder.encode(&u8s, 1, 1, ColorType::Rgb8)?;

    Ok(())
}
