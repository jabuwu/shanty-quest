use crate::common::prelude::*;
use crate::game::prelude::*;
use audio_plus::prelude::*;
use bevy::prelude::*;

pub struct DashAttackPlugin;

impl Plugin for DashAttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_component_child::<DashAttack, DashAttackSound>()
            .add_system(dash_attack_fire)
            .add_system(dash_attack_update)
            .add_system(dash_attack_sound);
    }
}

#[derive(Component, Default)]
pub struct DashAttack {
    pub shoot: bool,
}

#[derive(Component)]
pub struct Dash {
    pub velocity: Vec2,
    pub time_alive: f32,
}

#[derive(Component, Default)]
struct DashAttackSound;

fn dash_attack_sound(
    mut commands: Commands,
    mut ev_created: EventReader<ComponentChildCreatedEvent<DashAttackSound>>,
    asset_library: Res<AssetLibrary>,
) {
    for event in ev_created.iter() {
        commands.entity(event.entity).insert(AudioPlusSource::new(
            asset_library.sound_effects.sfx_placeholder_sound.clone(),
        ));
    }
}

fn dash_attack_fire(
    mut query: Query<(&mut DashAttack, &Boat, Entity, &Children), Without<Dash>>,
    mut sound_query: Query<&mut AudioPlusSource, With<DashAttackSound>>,
    mut commands: Commands,
) {
    for (mut dash_attack, boat, entity, children) in query.iter_mut() {
        if dash_attack.shoot {
            for child in children.iter() {
                if let Ok(mut sound) = sound_query.get_mut(*child) {
                    sound.play();
                }
            }
            commands.entity(entity).insert(Dash {
                velocity: boat.facing.to_vec() * 1000.,
                time_alive: 0.,
            });
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
        if dash.time_alive > 0.25 {
            commands.entity(entity).remove::<Dash>();
        }
    }
}
