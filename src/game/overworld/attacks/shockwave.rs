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
    velocity: Vec2,
}

fn shockwave_fire(
    mut query: Query<(&mut Shockwave, &GlobalTransform, &Boat)>,
    mut commands: Commands,
) {
    for (mut shockwave, global_transform, boat) in query.iter_mut() {
        if shockwave.shoot {
            let position = global_transform.translation().truncate();
            let velocity = boat.facing.to_vec() * 200.;
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Vec2::new(1., 1.).into(),
                        color: Color::WHITE,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Transform2::from_translation(position).with_depth(DEPTH_LAYER_SHOCKWAVE))
                .insert(ShockwaveWave {
                    time_alive: 0.,
                    velocity,
                })
                .insert(TimeToLive::new(0.5));
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
        transform.scale = Vec2::ONE * (64. + wave.time_alive * 800.);
        transform.translation += wave.velocity * time.delta_seconds();
        sprite.color.set_a(1. - wave.time_alive / 0.5);
    }
}
