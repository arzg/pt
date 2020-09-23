use std::ops::{Add, AddAssign, Mul};
use ultraviolet::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Rgb(pub Vec3);

impl Rgb {
    pub fn new(red: f32, green: f32, blue: f32) -> Self {
        Self(Vec3::new(red, green, blue))
    }

    pub fn iter(self, samples: u16) -> RgbIter {
        RgbIter {
            inner: self,
            idx: RgbIterIdx::Red,
            samples,
        }
    }
}

impl Add<Self> for Rgb {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for Rgb {
    fn add_assign(&mut self, rhs: Self) {
        self.0 = self.0 + rhs.0;
    }
}

impl Mul<f32> for Rgb {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * rhs)
    }
}

pub struct RgbIter {
    inner: Rgb,
    idx: RgbIterIdx,
    samples: u16,
}

impl Iterator for RgbIter {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let samples = f32::from(self.samples);
        let f32_to_u8 = |f: f32| {
            let corrected = (f / samples).sqrt();
            debug_assert!(corrected >= 0.0 && corrected <= 1.0);

            (corrected * 255.0).round() as u8
        };

        match self.idx {
            RgbIterIdx::Red => {
                self.idx = RgbIterIdx::Green;
                Some(f32_to_u8(self.inner.0.x))
            }
            RgbIterIdx::Green => {
                self.idx = RgbIterIdx::Blue;
                Some(f32_to_u8(self.inner.0.y))
            }
            RgbIterIdx::Blue => {
                self.idx = RgbIterIdx::Finished;
                Some(f32_to_u8(self.inner.0.z))
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
