#[derive(Copy, Clone)]
pub struct Rgb {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
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
