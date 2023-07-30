use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<state::GameState>().add_plugins((
            cutscenes::CutscenesPlugin,
            overworld::OverworldPlugin,
            town::TownPlugin,
            quests::QuestsPlugin,
            dead::DeadPlugin,
        ));
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
