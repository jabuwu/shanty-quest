use bevy::prelude::*;

pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(rubble::RubblePlugin);
    }
}

pub mod rubble;
