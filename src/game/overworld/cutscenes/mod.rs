use bevy::prelude::*;

pub struct CutscenesPlugin;

impl Plugin for CutscenesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(enter_town::EnterTownCutscenePlugin)
            .add_plugin(exit_town::ExitTownCutscenePlugin)
            .add_plugin(example_dialogue::ExampleDialogueCutscenePlugin)
            .add_plugin(death::DeathCutscenePlugin)
            .add_plugin(dangerous_seas::DangerousSeasCutscenePlugin);
    }
}

pub mod dangerous_seas;
pub mod death;
pub mod enter_town;
pub mod example_dialogue;
pub mod exit_town;
