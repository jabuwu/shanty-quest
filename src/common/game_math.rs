pub trait GameMath {
    fn lerp(&self, start: Self, end: Self) -> Self;
}

impl GameMath for f32 {
    fn lerp(&self, start: f32, end: f32) -> f32 {
        start + (end - start) * *self
    }
}
