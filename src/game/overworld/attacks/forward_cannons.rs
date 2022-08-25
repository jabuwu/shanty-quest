use crate::common::prelude::*;
use crate::game::prelude::*;
use audio_plus::prelude::*;
use bevy::prelude::*;

pub struct ForwardCannonsPlugin;

impl Plugin for ForwardCannonsPlugin {
    fn build(&self, app: &mut App) {
        app.add_component_child::<ForwardCannons, ForwardCannonsSound>()
            .add_system(forward_cannons_fire)
            .add_system(forward_cannon_ball_move)
            .add_system(forward_cannons_sound);
    }
}

#[derive(Component, Default)]
pub struct ForwardCannons {
    pub shoot: bool,
    pub hurt_flags: u32,
}

#[derive(Component)]
struct ForwardCannonBall {
    pub velocity: Vec2,
}

#[derive(Component, Default)]
struct ForwardCannonsSound;

fn forward_cannons_sound(
    mut commands: Commands,
    mut ev_created: EventReader<ComponentChildCreatedEvent<ForwardCannonsSound>>,
    asset_library: Res<AssetLibrary>,
) {
    for event in ev_created.iter() {
        commands
            .entity(event.entity)
            .insert_bundle(Transform2Bundle::default())
            .insert(AudioPlusSource::new(
                asset_library
                    .sound_effects
                    .sfx_overworld_attack_forward_cannons
                    .clone(),
            ));
    }
}

fn forward_cannons_fire(
    mut query: Query<(
        Entity,
        &mut ForwardCannons,
        &Boat,
        &GlobalTransform,
        &Children,
    )>,
    mut sound_query: Query<&mut AudioPlusSource, With<ForwardCannonsSound>>,
    mut commands: Commands,
) {
    for (boat_entity, mut forward_cannons, boat, global_transform, children) in query.iter_mut() {
        if forward_cannons.shoot {
            for child in children.iter() {
                if let Ok(mut sound) = sound_query.get_mut(*child) {
                    sound.play();
                }
            }
            for shoot_side in 0..2 {
                let forward = Vec2::from_angle(boat.direction);
                let mult = if shoot_side == 0 { 1. } else { -1. };
                let side = forward.perp() * mult;
                let position =
                    global_transform.translation().truncate() + forward * 40. + side * 15.;
                let velocity = forward * 1200.;
                let (scale, _, _) = global_transform.to_scale_rotation_translation();
                commands
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            custom_size: Vec2::new(8., 8.).into(),
                            color: Color::BLACK,
                            ..Default::default()
                        },
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
                        flags: forward_cannons.hurt_flags,
                        knockback_type: HurtboxKnockbackType::Velocity(velocity * 0.01),
                    })
                    .insert(YDepth::default())
                    .insert(ForwardCannonBall { velocity })
                    .insert(TimeToLive::new(1.0));
            }
        }
        forward_cannons.shoot = false;
    }
}

fn forward_cannon_ball_move(
    mut query: Query<(&mut Transform2, &ForwardCannonBall)>,
    time: Res<Time>,
) {
    for (mut transform, cannon_ball) in query.iter_mut() {
        transform.translation += cannon_ball.velocity * time.delta_seconds()
    }
}
