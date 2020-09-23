use ultraviolet::Vec3;

struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}
