use bevy::prelude::*;

#[derive(Component)]
pub struct Health {
    pub max: f32,
    pub value: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self { max, value: max }
    }

    pub fn new_with_max(value: f32, max: f32) -> Self {
        Self { max, value }
    }

    pub fn damage(&mut self, amt: f32) {
        self.value = (self.value - amt).clamp(0., self.max);
    }

    pub fn dead(&mut self) -> bool {
        self.value == 0.
    }
}
