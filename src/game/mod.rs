use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<state::GameState>()
            .add_plugin(overworld::OverworldPlugin)
            .add_plugin(town::TownPlugin);
    }
}

pub mod data;
pub mod overworld;
pub mod prelude;
pub mod state;
pub mod town;
