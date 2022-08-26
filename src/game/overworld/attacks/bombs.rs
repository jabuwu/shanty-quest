use crate::common::prelude::*;
use crate::game::prelude::*;
use audio_plus::prelude::*;
use bevy::prelude::*;

pub struct BombsPlugin;

impl Plugin for BombsPlugin {
    fn build(&self, app: &mut App) {
        app.add_component_child::<Bombs, BombsSound>()
            .add_system(bombs_fire)
            .add_system(bomb_move)
            .add_system(bomb_animate)
            .add_system(bombs_sound);
    }
}

#[derive(Component, Default)]
pub struct Bombs {
    pub shoot: bool,
    pub hurt_flags: u32,
    pub boss: bool,
}

#[derive(Component)]
struct Bomb {
    pub velocity: Vec2,
    pub life_time: f32,
    pub parent: Entity,
    pub hurt_flags: u32,
    pub boss: bool,
}

#[derive(Component, Default)]
struct BombsSound;

fn bombs_sound(
    mut commands: Commands,
    mut ev_created: EventReader<ComponentChildCreatedEvent<BombsSound>>,
    asset_library: Res<AssetLibrary>,
) {
    for event in ev_created.iter() {
        commands
            .entity(event.entity)
            .insert_bundle(Transform2Bundle::default())
            .insert(AudioPlusSource::new(
                asset_library
                    .sound_effects
                    .sfx_overworld_attack_bombs
                    .clone(),
            ));
    }
}

fn bombs_fire(
    mut query: Query<(Entity, &mut Bombs, &Boat, &GlobalTransform, &Children)>,
    mut sound_query: Query<&mut AudioPlusSource, With<BombsSound>>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for (boat_entity, mut bombs, boat, global_transform, children) in query.iter_mut() {
        if bombs.shoot {
            for child in children.iter() {
                if let Ok(mut sound) = sound_query.get_mut(*child) {
                    sound.play();
                }
            }
            let amt = if bombs.boss { 3 } else { 1 };
            for _ in 0..amt {
                let throw_direction =
                    Vec2::from_angle(rand::random::<f32>() * std::f32::consts::TAU);
                let position = global_transform.translation().truncate() + throw_direction * 100.;
                let (velocity_min, velocity_max) = if bombs.boss {
                    (100., 1500.)
                } else {
                    (200., 500.)
                };
                let velocity = throw_direction
                    * (velocity_min + rand::random::<f32>() * (velocity_max - velocity_min))
                    + boat.movement.clamp(Vec2::NEG_ONE, Vec2::ONE) * 150.;
                commands
                    .spawn_bundle(SpriteSheetBundle {
                        texture_atlas: asset_library.sprite_bomb_atlas.clone(),
                        ..Default::default()
                    })
                    .insert(
                        Transform2::from_translation(position)
                            .with_depth((DepthLayer::Entity, 0.0)),
                    )
                    .insert(YDepth::default())
                    .insert(Bomb {
                        velocity,
                        life_time: 1.75,
                        parent: boat_entity,
                        hurt_flags: bombs.hurt_flags,
                        boss: bombs.boss,
                    });
            }
        }
        bombs.shoot = false;
    }
}

fn bomb_move(
    mut query: Query<(Entity, &mut Transform2, &mut Bomb, &GlobalTransform)>,
    time: Res<Time>,
    mut commands: Commands,
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
                        size: Vec2::new(150., 150.),
                    },
                    for_entity: Some(bomb.parent),
                    auto_despawn: false,
                    flags: bomb.hurt_flags,
                    knockback_type: HurtboxKnockbackType::Difference(if bomb.boss {
                        20.
                    } else {
                        7.5
                    }),
                    damage: 3.,
                })
                .insert(YDepth::default())
                .insert(TimeToLive { seconds: 0.05 });
        }
    }
}

fn bomb_animate(mut query: Query<&mut TextureAtlasSprite, With<Bomb>>, time: Res<Time>) {
    for mut sprite in query.iter_mut() {
        let time = (time.time_since_startup().as_secs_f32() * 10.0) % 1.;
        if time > 0.5 {
            sprite.index = 1;
        } else {
            sprite.index = 0;
        }
    }
}
