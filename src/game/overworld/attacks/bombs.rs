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
            .add_system(bombs_sound);
    }
}

#[derive(Component, Default)]
pub struct Bombs {
    pub shoot: bool,
    pub hurt_flags: u32,
}

#[derive(Component)]
struct Bomb {
    pub velocity: Vec2,
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
            for shoot_side in 0..2 {
                let forward = Vec2::from_angle(boat.direction + std::f32::consts::PI);
                let mult = if shoot_side == 0 { 1. } else { -1. };
                let side = forward.perp() * mult;
                let position =
                    global_transform.translation().truncate() + forward * 40. + side * 15.;
                let velocity = forward * 200.;
                let (mut scale, _, _) = global_transform.to_scale_rotation_translation();
                scale *= 0.5;
                commands
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::BLACK,
                            ..Default::default()
                        },
                        texture: asset_library.sprite_bullet_note.clone(),
                        ..Default::default()
                    })
                    .insert(
                        Transform2::from_translation(position)
                            .with_depth((DepthLayer::Entity, 0.0))
                            .with_scale(scale.truncate()),
                    )
                    .insert(Hurtbox {
                        shape: CollisionShape::Rect {
                            size: Vec2::new(14., 14.),
                        },
                        for_entity: Some(boat_entity),
                        auto_despawn: true,
                        flags: bombs.hurt_flags,
                        knockback_type: HurtboxKnockbackType::Velocity(velocity * 0.01),
                    })
                    .insert(YDepth::default())
                    .insert(Bomb { velocity })
                    .insert(TimeToLive::new(1.0));
            }
        }
        bombs.shoot = false;
    }
}

fn bomb_move(mut query: Query<(&mut Transform2, &Bomb)>, time: Res<Time>) {
    for (mut transform, cannon_ball) in query.iter_mut() {
        transform.translation += cannon_ball.velocity * time.delta_seconds()
    }
}
