use bevy::prelude::*;

pub struct AttacksPlugin;

impl Plugin for AttacksPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(shotgun_cannons::ShotgunCannonsPlugin)
            .add_plugin(shockwave::ShockwavePlugin)
            .add_plugin(dash_attack::DashAttackPlugin);
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Attack {
    ShotgunCannons,
    Shockwave,
    DashAttack,
}

pub mod dash_attack;
pub mod shockwave;
pub mod shotgun_cannons;
