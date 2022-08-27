use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;

pub struct ExperiencePlugin;

impl Plugin for ExperiencePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ExperienceSpawnEvent>()
            .add_system(experience_spawn)
            .add_system(experience_consume);
    }
}

#[derive(Default, Clone, Copy)]
pub struct ExperienceSpawnEvent {
    pub position: Vec2,
}

#[derive(Component)]
pub struct Experience {
    velocity: Vec2,
}

pub fn experience_spawn(
    mut ev_spawn: EventReader<ExperienceSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for event in ev_spawn.iter() {
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Vec2::new(20., 20.).into(),
                    //color: Color::rgb_u8(255, 209, 22),
                    ..Default::default()
                },
                texture: asset_library.sprite_exp_particle.clone(),
                ..Default::default()
            })
            .insert(
                Transform2::from_translation(event.position).with_depth((DepthLayer::Entity, 0.)),
            )
            .insert(YDepth::default())
            .insert(Experience {
                velocity: Vec2::ZERO,
            });
    }
}

pub fn experience_consume(
    player_query: Query<&GlobalTransform, With<Player>>,
    mut experience_query: Query<(Entity, &mut Transform2, &mut Experience)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    let player_position = if let Ok(player_transform) = player_query.get_single() {
        player_transform.translation().truncate()
    } else {
        return;
    };
    for (entity, mut transform, mut experience) in experience_query.iter_mut() {
        let difference = player_position - transform.translation;
        if difference.length() < 200. {
            experience.velocity += difference.normalize() * 800. * time.delta_seconds();
        }
        if difference.length() < 100. {
            commands.entity(entity).despawn();
        }
        experience.velocity *= 0.1_f32.powf(time.delta_seconds());
        transform.translation += experience.velocity * time.delta_seconds();
    }
}
