use bevy::prelude::*;

pub struct CutscenesPlugin;

impl Plugin for CutscenesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            enter_town::EnterTownCutscenePlugin,
            exit_town::ExitTownCutscenePlugin,
            example_dialogue::ExampleDialogueCutscenePlugin,
            death::DeathCutscenePlugin,
            dangerous_seas::DangerousSeasCutscenePlugin,
        ));
    }
}

pub mod dangerous_seas;
pub mod death;
pub mod enter_town;
pub mod example_dialogue;
pub mod exit_town;
