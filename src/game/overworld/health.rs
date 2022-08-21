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
}
