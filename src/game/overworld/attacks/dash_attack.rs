use crate::{common::prelude::*, game::prelude::*};
use audio_plus::prelude::*;
use bevy::prelude::*;

pub struct DashAttackPlugin;

impl Plugin for DashAttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (dash_attack_fire, dash_attack_update));
    }
}

#[derive(Component, Default)]
pub struct DashAttack {
    pub shoot: bool,
    pub hurt_flags: u32,
}

#[derive(Component)]
pub struct Dash {
    pub velocity: Vec2,
    pub time_alive: f32,
}

fn dash_attack_fire(
    mut query: Query<(&mut DashAttack, &Boat, Entity, &GlobalTransform), Without<Dash>>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for (mut dash_attack, boat, entity, global_transform) in query.iter_mut() {
        if dash_attack.shoot {
            let audio_entity = commands
                .spawn((
                    Transform2Bundle {
                        transform2: Transform2::from_translation(
                            global_transform.translation().truncate(),
                        ),
                        ..Default::default()
                    },
                    AudioPlusSource::new(asset_library.sound_effects.sfx_overworld_dash.clone())
                        .as_playing(),
                    TimeToLive { seconds: 3. },
                ))
                .id();
            commands
                .entity(entity)
                .insert(Dash {
                    velocity: boat.facing.to_vec() * 750.,
                    time_alive: 0.,
                })
                .add_child(audio_entity);
            dash_attack.shoot = false;
        }
    }
}

fn dash_attack_update(
    mut query: Query<(&mut Dash, Entity)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (mut dash, entity) in query.iter_mut() {
        dash.time_alive += time.delta_seconds();
        if dash.time_alive > 0.2 {
            commands.entity(entity).remove::<Dash>();
        }
    }
}
