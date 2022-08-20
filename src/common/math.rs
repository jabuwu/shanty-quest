pub trait Lerp {
    fn lerp(&self, start: Self, end: Self) -> Self;
}

impl Lerp for f32 {
    fn lerp(&self, start: f32, end: f32) -> f32 {
        start + (end - start) * *self
    }
}

pub struct Rect<T> {
    pub left: T,
    pub right: T,
    pub bottom: T,
    pub top: T,
}
