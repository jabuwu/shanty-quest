use bevy::prelude::*;

pub struct AttacksPlugin;

impl Plugin for AttacksPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            forward_cannons::ForwardCannonsPlugin,
            shotgun_cannons::ShotgunCannonsPlugin,
            shockwave::ShockwavePlugin,
            dash_attack::DashAttackPlugin,
            bombs::BombsPlugin,
            kraken::KrakenPlugin,
        ));
    }
}

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Attacks {
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
