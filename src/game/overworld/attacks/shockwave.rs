use crate::common::prelude::*;
use crate::game::prelude::*;
use audio_plus::prelude::*;
use bevy::prelude::*;

pub struct ShockwavePlugin;

impl Plugin for ShockwavePlugin {
    fn build(&self, app: &mut App) {
        app.add_component_child::<Shockwave, ShockwaveSound>()
            .add_system(shockwave_fire)
            .add_system(shockwave_update)
            .add_system(shockwave_sound);
    }
}

#[derive(Component, Default)]
pub struct Shockwave {
    pub shoot: bool,
    pub hurt_flags: u32,
}

#[derive(Component, Default)]
struct ShockwaveWave {
    time_alive: f32,
    _velocity: Vec2,
}

#[derive(Component, Default)]
struct ShockwaveSound;

fn shockwave_sound(
    mut commands: Commands,
    mut ev_created: EventReader<ComponentChildCreatedEvent<ShockwaveSound>>,
    asset_library: Res<AssetLibrary>,
) {
    for event in ev_created.iter() {
        commands.entity(event.entity).insert(AudioPlusSource::new(
            asset_library.sound_effects.sfx_placeholder_sound.clone(),
        ));
    }
}

fn shockwave_fire(
    mut query: Query<(&mut Shockwave, &Boat, Entity, &Children)>,
    mut sound_query: Query<&mut AudioPlusSource, With<ShockwaveSound>>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for (mut shockwave, boat, entity, children) in query.iter_mut() {
        if shockwave.shoot {
            for child in children.iter() {
                if let Ok(mut sound) = sound_query.get_mut(*child) {
                    sound.play();
                }
            }
            let velocity = boat.facing.to_vec() * 200.;
            let child_entity = commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Vec2::new(1., 1.).into(),
                        color: Color::WHITE,
                        ..Default::default()
                    },
                    texture: asset_library.sprite_shockwave_vfx.clone(),
                    ..Default::default()
                })
                .insert(Transform2::new().with_depth(DEPTH_LAYER_SHOCKWAVE))
                .insert(ShockwaveWave {
                    time_alive: 0.,
                    _velocity: velocity,
                })
                .insert(TimeToLive::new(0.75))
                .id();
            commands.entity(entity).add_child(child_entity);
        }
        shockwave.shoot = false;
    }
}

fn shockwave_update(
    mut query: Query<(&mut ShockwaveWave, &mut Transform2, &mut Sprite)>,
    time: Res<Time>,
) {
    for (mut wave, mut transform, mut sprite) in query.iter_mut() {
        wave.time_alive += time.delta_seconds();
        transform.scale = Vec2::ONE * (32. + wave.time_alive * 2000.);
        //transform.translation += wave.velocity * time.delta_seconds();
        sprite.color.set_a(1. - wave.time_alive / 0.75);
    }
}
