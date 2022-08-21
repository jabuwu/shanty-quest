use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;

pub struct ShotgunCannonsPlugin;

impl Plugin for ShotgunCannonsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(shotgun_cannons_fire)
            .add_system(shotgun_cannon_ball_move);
    }
}

#[derive(Component, Default)]
pub struct ShotgunCannons {
    pub shoot: bool,
}

#[derive(Component)]
struct ShotgunCannonBall {
    pub velocity: Vec2,
}

fn shotgun_cannons_fire(
    mut query: Query<(&mut ShotgunCannons, &Boat, &GlobalTransform)>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for (mut shotgun_cannons, boat, global_transform) in query.iter_mut() {
        if shotgun_cannons.shoot {
            for shoot_side in 0..2 {
                let forward = boat.facing.to_vec();
                let mult = if shoot_side == 0 { 1. } else { -1. };
                let side = forward.perp() * mult;
                for i in -1..=1 {
                    let mut angle = Vec2::X.angle_between(side);
                    angle -= std::f32::consts::PI * 0.1 * i as f32 * mult;
                    let position = global_transform.translation().truncate()
                        + forward * 20. * i as f32
                        + side * 50.;
                    let velocity = Vec2::from_angle(angle) * 900.;
                    commands
                        .spawn_bundle(SpriteBundle {
                            sprite: Sprite {
                                custom_size: Vec2::new(8., 8.).into(),
                                color: Color::BLACK,
                                ..Default::default()
                            },
                            texture: asset_library.sprite_bullet_note.clone(),
                            ..Default::default()
                        })
                        .insert(
                            Transform2::from_translation(position)
                                .with_depth((DepthLayer::Entity, 0.5)),
                        )
                        .insert(ShotgunCannonBall { velocity })
                        .insert(TimeToLive::new(0.5));
                }
            }
        }
        shotgun_cannons.shoot = false;
    }
}

fn shotgun_cannon_ball_move(
    mut query: Query<(&mut Transform2, &ShotgunCannonBall)>,
    time: Res<Time>,
) {
    for (mut transform, cannon_ball) in query.iter_mut() {
        transform.translation += cannon_ball.velocity * time.delta_seconds()
    }
}
