use std::ops::{Add, Mul};
use ultraviolet::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Rgb(Vec3);

impl Rgb {
    pub fn new(red: f32, green: f32, blue: f32) -> Self {
        Self(Vec3::new(red, green, blue))
    }
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

impl Add<Self> for Rgb {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
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
