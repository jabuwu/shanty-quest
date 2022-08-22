use crate::common::prelude::*;
use bevy::prelude::*;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DamageEvent>().add_system(damage_check);
    }
}
