use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<state::GameState>()
            .add_plugin(cutscenes::CutscenesPlugin)
            .add_plugin(overworld::OverworldPlugin)
            .add_plugin(town::TownPlugin)
            .add_plugin(quests::QuestsPlugin)
            .add_plugin(dead::DeadPlugin);
    }
}

pub mod all_dialogue;
pub mod cutscenes;
pub mod data;
pub mod dead;
pub mod overworld;
pub mod prelude;
pub mod quests;
pub mod state;
pub mod town;
