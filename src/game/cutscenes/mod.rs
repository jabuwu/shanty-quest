use bevy::prelude::*;

pub struct CutscenesPlugin;

impl Plugin for CutscenesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((intro::IntroCutscenePlugin, outro::OutroCutscenePlugin));
    }
}

pub mod intro;
pub mod outro;
