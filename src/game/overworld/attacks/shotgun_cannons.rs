use crate::common::prelude::*;
use crate::game::prelude::*;
use audio_plus::prelude::*;
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
    pub hurt_flags: u32,
    pub level: ShotgunCannonsLevel,
}

#[derive(Default)]
pub struct ShotgunCannonsLevel(pub u32);

impl ShotgunCannonsLevel {
    fn stats(&self) -> ShotgunCannonsStats {
        if self.0 == 6 {
            // boss stats
            ShotgunCannonsStats {
                damage: 10.,
                time_to_live: 0.37,
                scale: 1.,
                angle: 0.1,
                count: 1,
            }
        } else {
            let level = self.0 as f32;
            ShotgunCannonsStats {
                damage: level / 3.,
                time_to_live: 0.37,
                scale: 1. + level / 5.,
                angle: if self.0 == 5 { 0.1 } else { 0.1 + level / 30. },
                count: if self.0 == 5 { 2 } else { 1 },
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct ShotgunCannonsStats {
    damage: f32,
    time_to_live: f32,
    scale: f32,
    angle: f32,
    count: i32,
}

#[derive(Component)]
struct ShotgunCannonBall {
    pub velocity: Vec2,
}

fn shotgun_cannons_fire(
    mut query: Query<(Entity, &mut ShotgunCannons, &Boat, &GlobalTransform)>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for (boat_entity, mut shotgun_cannons, boat, global_transform) in query.iter_mut() {
        if shotgun_cannons.shoot {
            let stats = shotgun_cannons.level.stats();
            commands
                .spawn_bundle(Transform2Bundle {
                    transform2: Transform2::from_translation(
                        global_transform.translation().truncate(),
                    ),
                    ..Default::default()
                })
                .insert(
                    AudioPlusSource::new(
                        asset_library
                            .sound_effects
                            .sfx_overworld_attack_shotgun_cannons
                            .clone(),
                    )
                    .as_playing(),
                )
                .insert(TimeToLive { seconds: 3. });
            for shoot_side in 0..2 {
                let forward = Vec2::from_angle(boat.direction);
                let mult = if shoot_side == 0 { 1. } else { -1. };
                let side = forward.perp() * mult;
                for i in -stats.count..=stats.count {
                    let mut angle = Vec2::X.angle_between(side);
                    angle -= std::f32::consts::PI * stats.angle * i as f32 * mult;
                    let position = global_transform.translation().truncate()
                        + forward * 20. * i as f32
                        + side * 50.;
                    let velocity = Vec2::from_angle(angle) * 900.;
                    let (mut scale, _, _) = global_transform.to_scale_rotation_translation();
                    scale *= stats.scale * 0.5;
                    commands
                        .spawn_bundle(SpriteBundle {
                            sprite: Sprite {
                                color: Color::BLACK,
                                ..Default::default()
                            },
                            texture: asset_library.sprite_bullet_note.clone(),
                            ..Default::default()
                        })
                        .insert(Hurtbox {
                            shape: CollisionShape::Rect {
                                size: Vec2::new(32., 32.) * stats.scale,
                            },
                            for_entity: Some(boat_entity),
                            auto_despawn: true,
                            flags: shotgun_cannons.hurt_flags,
                            knockback_type: HurtboxKnockbackType::None,
                            damage: stats.damage,
                        })
                        .insert(
                            Transform2::from_translation(position)
                                .with_depth((DepthLayer::Entity, 0.5))
                                .with_scale(scale.truncate()),
                        )
                        .insert(ShotgunCannonBall { velocity })
                        .insert(TimeToLive::new(stats.time_to_live));
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
