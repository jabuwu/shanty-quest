use bevy::prelude::*;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ldtk::LdtkAssetPlugin);
    }
}

pub mod ldtk;
