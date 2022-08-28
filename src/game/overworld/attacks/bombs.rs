use crate::common::prelude::*;
use crate::game::prelude::*;
use audio_plus::prelude::*;
use bevy::prelude::*;

pub struct BombsPlugin;

impl Plugin for BombsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(bombs_fire)
            .add_system(bomb_move)
            .add_system(bomb_animate);
    }
}

#[derive(Component, Default)]
pub struct Bombs {
    pub shoot: bool,
    pub hurt_flags: u32,
    pub level: BombsLevel,
}

#[derive(Default)]
pub struct BombsLevel(pub u32);

impl BombsLevel {
    fn stats(&self) -> BombsStats {
        if self.0 == 6 {
            // boss stats
            BombsStats {
                damage: 2.5,
                knockback_intensity: 20.,
                spawn_amount: 3,
                velocity_min: 100.,
                velocity_max: 1500.,
            }
        } else {
            BombsStats {
                damage: 3.,
                knockback_intensity: 7.5,
                spawn_amount: self.0,
                velocity_min: 200.,
                velocity_max: 500.,
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct BombsStats {
    damage: f32,
    knockback_intensity: f32,
    spawn_amount: u32,
    velocity_min: f32,
    velocity_max: f32,
}

#[derive(Component)]
struct Bomb {
    pub velocity: Vec2,
    pub life_time: f32,
    pub life_time_max: f32,
    pub parent: Entity,
    pub hurt_flags: u32,
    pub stats: BombsStats,
}

fn bombs_fire(
    mut query: Query<(Entity, &mut Bombs, &Boat, &GlobalTransform)>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for (boat_entity, mut bombs, boat, global_transform) in query.iter_mut() {
        if bombs.shoot {
            let stats = bombs.level.stats();
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
                            .sfx_overworld_attack_bombs
                            .clone(),
                    )
                    .as_playing(),
                )
                .insert(TimeToLive { seconds: 3. });
            for _ in 0..stats.spawn_amount {
                let throw_direction =
                    Vec2::from_angle(rand::random::<f32>() * std::f32::consts::TAU);
                let position = global_transform.translation().truncate() + throw_direction * 100.;
                let velocity = throw_direction
                    * (stats.velocity_min
                        + rand::random::<f32>() * (stats.velocity_max - stats.velocity_min))
                    + boat.movement.clamp(Vec2::NEG_ONE, Vec2::ONE) * 150.;
                commands
                    .spawn_bundle(SpriteSheetBundle {
                        texture_atlas: asset_library.sprite_bomb_atlas.clone(),
                        ..Default::default()
                    })
                    .insert(
                        Transform2::from_translation(position)
                            .with_depth((DepthLayer::Entity, 0.0))
                            .with_scale(Vec2::ONE * 0.75),
                    )
                    .insert(YDepth::default())
                    .insert(Bomb {
                        velocity,
                        life_time: 1.75,
                        life_time_max: 1.75,
                        parent: boat_entity,
                        hurt_flags: bombs.hurt_flags,
                        stats,
                    });
            }
            commands
                .spawn_bundle(TransformBundle::default())
                .insert(Transform2::from_translation(
                    global_transform.translation().truncate(),
                ))
                .insert(
                    AudioPlusSource::new(
                        asset_library
                            .sound_effects
                            .sfx_overworld_attack_bomb_throw
                            .clone(),
                    )
                    .as_playing(),
                )
                .insert(TimeToLive { seconds: 4. });
        }
        bombs.shoot = false;
    }
}

fn bomb_move(
    mut query: Query<(Entity, &mut Transform2, &mut Bomb, &GlobalTransform)>,
    time: Res<Time>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for (entity, mut transform, mut bomb, global_transform) in query.iter_mut() {
        transform.translation += bomb.velocity * time.delta_seconds();
        bomb.velocity *= 0.2_f32.powf(time.delta_seconds());
        bomb.life_time -= time.delta_seconds();
        if bomb.life_time < 0. {
            commands.entity(entity).despawn();
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Vec2::new(150., 150.).into(),
                        color: Color::RED,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(
                    Transform2::from_translation(global_transform.translation().truncate())
                        .with_depth((DepthLayer::Entity, 0.)),
                )
                .insert(Hurtbox {
                    shape: CollisionShape::Rect {
                        size: Vec2::new(180., 180.),
                    },
                    for_entity: Some(bomb.parent),
                    auto_despawn: false,
                    flags: bomb.hurt_flags,
                    knockback_type: HurtboxKnockbackType::Difference(
                        bomb.stats.knockback_intensity,
                    ),
                    damage: bomb.stats.damage,
                })
                .insert(YDepth::default())
                .insert(TimeToLive { seconds: 0.05 });
            commands
                .spawn_bundle(TransformBundle::default())
                .insert(Transform2::from_translation(
                    global_transform.translation().truncate(),
                ))
                .insert(
                    AudioPlusSource::new(
                        asset_library
                            .sound_effects
                            .sfx_overworld_attack_bomb_explode
                            .clone(),
                    )
                    .as_playing(),
                )
                .insert(TimeToLive { seconds: 4. });
        }
    }
}

fn bomb_animate(mut query: Query<(&mut TextureAtlasSprite, &Bomb)>) {
    for (mut sprite, bomb) in query.iter_mut() {
        let time = (bomb.life_time * 10.) % 1.;
        let flash = if bomb.life_time < bomb.life_time_max * 0.4 {
            let flash_mod = ((bomb.life_time * 4.) as i32).max(2);
            ((bomb.life_time * 8.5) as i32) % flash_mod == 0
        } else {
            false
        };
        if time > 0.5 {
            sprite.index = if flash { 3 } else { 1 };
        } else {
            sprite.index = if flash { 2 } else { 0 };
        }
    }
}
