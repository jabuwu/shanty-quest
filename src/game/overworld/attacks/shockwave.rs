use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;

pub struct ShockwavePlugin;

impl Plugin for ShockwavePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(shockwave_fire).add_system(shockwave_update);
    }
}

#[derive(Component, Default)]
pub struct Shockwave {
    pub shoot: bool,
}

#[derive(Component, Default)]
struct ShockwaveWave {
    time_alive: f32,
    _velocity: Vec2,
}

fn shockwave_fire(
    mut query: Query<(&mut Shockwave, &Boat, Entity)>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for (mut shockwave, boat, entity) in query.iter_mut() {
        if shockwave.shoot {
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
