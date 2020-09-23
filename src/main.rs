use image::png::PngEncoder;
use image::ColorType;
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

#[derive(Copy, Clone)]
struct Rgb {
    red: f32,
    green: f32,
    blue: f32,
}

impl IntoIterator for Rgb {
    type Item = u8;
    type IntoIter = RgbIter;

    fn into_iter(self) -> Self::IntoIter {
        RgbIter {
            inner: self,
            idx: RgbIterIdx::Red,
        }
    }
}

struct RgbIter {
    inner: Rgb,
    idx: RgbIterIdx,
}

impl Iterator for RgbIter {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let f32_to_u8 = |f: f32| {
            debug_assert!(f >= 0.0 && f <= 1.0);
            (f / 1.0 * 255.0).round() as u8
        };

        match self.idx {
            RgbIterIdx::Red => {
                self.idx = RgbIterIdx::Green;
                Some(f32_to_u8(self.inner.red))
            }
            RgbIterIdx::Green => {
                self.idx = RgbIterIdx::Blue;
                Some(f32_to_u8(self.inner.green))
            }
            RgbIterIdx::Blue => {
                self.idx = RgbIterIdx::Finished;
                Some(f32_to_u8(self.inner.blue))
            }
            RgbIterIdx::Finished => None,
        }
    }
}

enum RgbIterIdx {
    Red,
    Green,
    Blue,
    Finished,
}
