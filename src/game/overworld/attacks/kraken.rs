use crate::common::prelude::*;
use crate::game::prelude::*;
use audio_plus::prelude::*;
use bevy::prelude::*;

pub struct KrakenPlugin;

impl Plugin for KrakenPlugin {
    fn build(&self, app: &mut App) {
        app.add_component_child::<Kraken, KrakenSound>()
            .add_system(kraken_fire)
            .add_system(tentacle_update)
            .add_system(tentacle_animate)
            .add_system(kraken_sound);
    }
}

#[derive(Component, Default)]
pub struct Kraken {
    pub shoot: bool,
    pub hurt_flags: u32,
}

#[derive(Component)]
struct Tentacle {
    pub submerge_time: f32,
    pub submerge_time_max: f32,
    pub spawned_hurtbox: bool,
    pub parent: Entity,
    pub hurt_flags: u32,
}

#[derive(Component, Default)]
struct KrakenSound;

fn kraken_sound(
    mut commands: Commands,
    mut ev_created: EventReader<ComponentChildCreatedEvent<KrakenSound>>,
    asset_library: Res<AssetLibrary>,
) {
    for event in ev_created.iter() {
        commands
            .entity(event.entity)
            .insert_bundle(Transform2Bundle::default())
            .insert(AudioPlusSource::new(
                asset_library
                    .sound_effects
                    .sfx_overworld_attack_kraken
                    .clone(),
            ));
    }
}

fn kraken_fire(
    mut query: Query<(Entity, &mut Kraken, &GlobalTransform, &Children)>,
    mut sound_query: Query<&mut AudioPlusSource, With<KrakenSound>>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for (boat_entity, mut kraken, global_transform, children) in query.iter_mut() {
        if kraken.shoot {
            for child in children.iter() {
                if let Ok(mut sound) = sound_query.get_mut(*child) {
                    sound.play();
                }
            }
            for _ in 0..2 {
                let forward = Vec2::from_angle(rand::random::<f32>() * std::f32::consts::TAU);
                let position = global_transform.translation().truncate()
                    + forward * (150. + rand::random::<f32>() * 400.);
                let (scale, _, _) = global_transform.to_scale_rotation_translation();
                let submerge_time = 1.0;
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
                    .insert(YDepth::default())
                    .insert(Tentacle {
                        submerge_time,
                        submerge_time_max: submerge_time,
                        spawned_hurtbox: false,
                        parent: boat_entity,
                        hurt_flags: kraken.hurt_flags,
                    })
                    .insert(TimeToLive::new(3.0));
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
        if tentacle.submerge_time <= 0. && !tentacle.spawned_hurtbox {
            commands.entity(entity).insert(Hurtbox {
                shape: CollisionShape::Rect {
                    size: Vec2::new(32., 48.),
                },
                for_entity: Some(tentacle.parent),
                auto_despawn: true,
                flags: tentacle.hurt_flags,
                knockback_type: HurtboxKnockbackType::Difference(25.),
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
            if time > 0.65 {
                sprite.index = 3;
            } else {
                sprite.index = 2;
            }
        }
    }
}
