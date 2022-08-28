use crate::common::prelude::*;
use crate::game::prelude::*;
use audio_plus::prelude::*;
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
    pub hurt_flags: u32,
    pub level: ShockwaveLevel,
}

#[derive(Default)]
pub struct ShockwaveLevel(pub u32);

impl ShockwaveLevel {
    fn stats(&self) -> ShockwaveStats {
        if self.0 == 6 {
            // boss stats
            ShockwaveStats {
                damage: 1.,
                knockback_intensity: 15.,
                scale: 0.7,
            }
        } else {
            let level = self.0 as f32;
            ShockwaveStats {
                damage: 0.25 * level,
                knockback_intensity: 5.,
                scale: 1. + level * 0.1,
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct ShockwaveStats {
    damage: f32,
    knockback_intensity: f32,
    scale: f32,
}

#[derive(Component)]
struct ShockwaveWave {
    time_alive: f32,
    stats: ShockwaveStats,
}

#[derive(Component, Default)]
struct ShockwaveSprite;

fn shockwave_fire(
    mut query: Query<(&mut Shockwave, Entity, &GlobalTransform)>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for (mut shockwave, entity, global_transform) in query.iter_mut() {
        if shockwave.shoot {
            let stats = shockwave.level.stats();
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
                            .sfx_overworld_attack_shockwave
                            .clone(),
                    )
                    .as_playing(),
                )
                .insert(TimeToLive { seconds: 3. });
            let child_entity = commands
                .spawn_bundle(Transform2Bundle {
                    ..Default::default()
                })
                .insert_bundle(VisibilityBundle::default())
                .insert(ShockwaveWave {
                    time_alive: 0.,
                    stats,
                })
                .insert(TimeToLive::new(0.75))
                .with_children(|parent| {
                    parent
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
                        .insert(ShockwaveSprite);
                    parent
                        .spawn_bundle(Transform2Bundle {
                            ..Default::default()
                        })
                        .insert(Transform2::new().with_depth((DepthLayer::Front, 0.98)))
                        .insert(Hurtbox {
                            shape: CollisionShape::Rect {
                                size: Vec2::new(400., 400.) * stats.scale,
                            },
                            for_entity: Some(entity),
                            auto_despawn: false,
                            flags: shockwave.hurt_flags,
                            knockback_type: HurtboxKnockbackType::Difference(
                                stats.knockback_intensity,
                            ),
                            damage: stats.damage,
                        })
                        .insert(TimeToLive { seconds: 0.05 });
                })
                .id();
            commands.entity(entity).add_child(child_entity);
        }
        shockwave.shoot = false;
    }
}

fn shockwave_update(
    mut query: Query<(&mut ShockwaveWave, &Children)>,
    mut child_query: Query<(&mut Transform2, &mut Sprite), With<ShockwaveSprite>>,
    time: Res<Time>,
) {
    for (mut wave, children) in query.iter_mut() {
        wave.time_alive += time.delta_seconds();
        for child in children.iter() {
            if let Ok((mut transform, mut sprite)) = child_query.get_mut(*child) {
                transform.scale = Vec2::ONE * (32. + wave.time_alive * 2000.) * wave.stats.scale;
                sprite.color.set_a(1. - wave.time_alive / 0.75);
            }
        }
    }
}
