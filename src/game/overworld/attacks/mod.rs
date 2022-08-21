use bevy::prelude::*;

pub struct AttacksPlugin;

impl Plugin for AttacksPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(shotgun_cannons::ShotgunCannonsPlugin)
            .add_plugin(shockwave::ShockwavePlugin);
    }
}

pub mod shockwave;
pub mod shotgun_cannons;
