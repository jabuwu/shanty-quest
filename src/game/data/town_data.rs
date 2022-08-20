use bevy::prelude::*;

#[derive(Clone)]
pub struct TownData {
    pub name: String,
    pub position: Vec2,
}

impl Default for TownData {
    fn default() -> Self {
        Self {
            name: "Dummy Town".to_owned(),
            position: Vec2::new(0., 0.),
        }
    }
}
