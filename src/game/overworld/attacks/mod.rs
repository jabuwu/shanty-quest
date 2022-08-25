use bevy::prelude::*;

pub struct AttacksPlugin;

impl Plugin for AttacksPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(forward_cannons::ForwardCannonsPlugin)
            .add_plugin(shotgun_cannons::ShotgunCannonsPlugin)
            .add_plugin(shockwave::ShockwavePlugin)
            .add_plugin(dash_attack::DashAttackPlugin)
            .add_plugin(bombs::BombsPlugin)
            .add_plugin(kraken::KrakenPlugin);
    }
}

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub struct SpecialAttack {
    pub forward_cannons: u32,
    pub shotgun_cannons: u32,
    pub shockwave: u32,
    pub bombs: u32,
    pub kraken: u32,
}

pub mod bombs;
pub mod dash_attack;
pub mod forward_cannons;
pub mod kraken;
pub mod shockwave;
pub mod shotgun_cannons;
