use bevy::prelude::*;

pub struct CutscenesPlugin;

impl Plugin for CutscenesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(intro::IntroCutscenePlugin)
            .add_plugin(outro::OutroCutscenePlugin);
    }
}

pub mod intro;
pub mod outro;
