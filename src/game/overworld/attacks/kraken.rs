use crate::common::prelude::*;
use crate::game::prelude::*;
use audio_plus::prelude::*;
use bevy::prelude::*;

pub struct KrakenPlugin;

impl Plugin for KrakenPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(kraken_fire)
            .add_system(tentacle_update)
            .add_system(tentacle_animate);
    }
}

#[derive(Component, Default)]
pub struct Kraken {
    pub shoot: bool,
    pub hurt_flags: u32,
    pub level: KrakenLevel,
}

#[derive(Default)]
pub struct KrakenLevel(pub u32);

impl KrakenLevel {
    fn stats(&self) -> KrakenStats {
        if self.0 == 6 {
            // boss stats
            KrakenStats {
                damage: 1.,
                close_tentacles: 1,
                far_tentacles: 5,
                far_tentacle_distance_min: 150.,
                far_tentacle_distance_max: 1650.,
                knockback_intensity: 10.,
            }
        } else {
            KrakenStats {
                damage: 1.,
                close_tentacles: 0,
                far_tentacles: 6,
                far_tentacle_distance_min: 150.,
                far_tentacle_distance_max: 500.,
                knockback_intensity: 5.,
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct KrakenStats {
    damage: f32,
    close_tentacles: u32,
    far_tentacles: u32,
    far_tentacle_distance_min: f32,
    far_tentacle_distance_max: f32,
    knockback_intensity: f32,
}

#[derive(Component)]
struct Tentacle {
    pub submerge_time: f32,
    pub submerge_time_max: f32,
    pub spawned_hurtbox: bool,
    pub parent: Entity,
    pub hurt_flags: u32,
    pub time_to_live: f32,
    pub stats: KrakenStats,
}

fn kraken_fire(
    mut query: Query<(Entity, &mut Kraken, &GlobalTransform)>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for (boat_entity, mut kraken, global_transform) in query.iter_mut() {
        if kraken.shoot {
            let stats = kraken.level.stats();
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
                            .sfx_overworld_attack_kraken
                            .clone(),
                    )
                    .as_playing(),
                )
                .insert(TimeToLive { seconds: 3. });
            for i in 0..(stats.close_tentacles + stats.far_tentacles) {
                let close_tentacle = i < stats.close_tentacles;
                let (distance_min, distance_max) = if close_tentacle {
                    (100., 100.)
                } else {
                    (
                        stats.far_tentacle_distance_min,
                        stats.far_tentacle_distance_max,
                    )
                };
                let forward = Vec2::from_angle(rand::random::<f32>() * std::f32::consts::TAU);
                let position = global_transform.translation().truncate()
                    + forward
                        * (distance_min + rand::random::<f32>() * (distance_max - distance_min));
                let (scale, _, _) = global_transform.to_scale_rotation_translation();
                let submerge_time = if close_tentacle { 0. } else { 1.0 };
                commands
                    .spawn_bundle(SpriteSheetBundle {
                        texture_atlas: asset_library.sprite_tentacle_atlas.clone(),
                        ..Default::default()
                    })
                    .insert(
                        Transform2::from_translation(position)
                            .with_depth((DepthLayer::Entity, 0.0))
                            .with_scale(scale.truncate()),
                    )
                    .insert(YDepth { offset: 10. })
                    .insert(Tentacle {
                        submerge_time,
                        submerge_time_max: submerge_time,
                        spawned_hurtbox: false,
                        parent: boat_entity,
                        hurt_flags: kraken.hurt_flags,
                        time_to_live: if close_tentacle { 1.5 } else { 3.0 },
                        stats,
                    });
            }
        }
        kraken.shoot = false;
    }
}

fn tentacle_update(
    mut query: Query<(Entity, &mut Tentacle)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut tentacle) in query.iter_mut() {
        tentacle.submerge_time -= time.delta_seconds();
        tentacle.time_to_live -= time.delta_seconds();
        if tentacle.time_to_live < 0. && tentacle.spawned_hurtbox {
            commands.entity(entity).despawn();
        }
        if tentacle.submerge_time <= 0. && !tentacle.spawned_hurtbox {
            commands.entity(entity).insert(Hurtbox {
                shape: CollisionShape::Rect {
                    size: Vec2::new(32., 48.),
                },
                for_entity: Some(tentacle.parent),
                auto_despawn: false,
                flags: tentacle.hurt_flags,
                knockback_type: HurtboxKnockbackType::Difference(
                    tentacle.stats.knockback_intensity,
                ),
                damage: tentacle.stats.damage,
            });
            tentacle.spawned_hurtbox = true;
        }
    }
}

fn tentacle_animate(mut query: Query<(&mut TextureAtlasSprite, &Tentacle)>, time: Res<Time>) {
    for (mut sprite, tentacle) in query.iter_mut() {
        if tentacle.submerge_time > 0. {
            if tentacle.submerge_time > tentacle.submerge_time_max * 0.25 {
                sprite.index = 0;
            } else {
                sprite.index = 1;
            }
        } else {
            let time = (time.time_since_startup().as_secs_f32() * 3.) % 1.;
            if time > 0.5 {
                sprite.index = 3;
            } else {
                sprite.index = 2;
            }
        }
    }
}
