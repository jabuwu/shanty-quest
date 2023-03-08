use crate::common::prelude::*;
use crate::game::prelude::*;
use audio_plus::prelude::*;
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
    pub amount: f32,
    pub position: Vec2,
    pub count: u32,
    pub infinite_distance: bool,
}

#[derive(Component)]
pub struct Experience {
    amount: f32,
    velocity: Vec2,
    infinite_distance: bool,
}

pub fn experience_spawn(
    mut ev_spawn: EventReader<ExperienceSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for event in ev_spawn.iter() {
        for _ in 0..event.count {
            let angle = Vec2::from_angle(rand::random::<f32>() * std::f32::consts::TAU);
            let velocity = angle * (50. + rand::random::<f32>() * 200.);
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Vec2::new(10., 10.).into(),
                        ..Default::default()
                    },
                    texture: asset_library.sprite_exp_particle.clone(),
                    ..Default::default()
                },
                Transform2::from_translation(event.position + angle * 20.)
                    .with_depth(DEPTH_LAYER_EXPERIENCE)
                    .with_scale(Vec2::ONE * (1. + event.amount / 2.5)),
                Experience {
                    amount: event.amount,
                    velocity,
                    infinite_distance: event.infinite_distance,
                },
            ));
        }
    }
}

pub fn experience_consume(
    player_query: Query<(Entity, &GlobalTransform), With<Player>>,
    mut experience_query: Query<(Entity, &mut Transform2, &mut Experience)>,
    time: Res<Time>,
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut ev_level_up: EventWriter<LevelUpSpawnEvent>,
    asset_library: Res<AssetLibrary>,
) {
    let (player_entity, player_position) =
        if let Ok((player_entity, player_transform)) = player_query.get_single() {
            (player_entity, player_transform.translation().truncate())
        } else {
            return;
        };
    for (entity, mut transform, mut experience) in experience_query.iter_mut() {
        let difference = player_position - transform.translation;
        if difference.length() < 200. || experience.infinite_distance {
            experience.velocity += difference.normalize() * 1400. * time.delta_seconds();
        }
        if difference.length() < 50. {
            let sound = commands
                .spawn((
                    Transform2Bundle::default(),
                    AudioPlusSource::new(
                        asset_library.sound_effects.sfx_overworld_experience.clone(),
                    )
                    .as_playing(),
                    TimeToLive { seconds: 3. },
                ))
                .id();
            commands.entity(player_entity).add_child(sound);
            if game_state.add_experience(experience.amount) {
                game_state.skill_points += 1;
                ev_level_up.send_default();
            }
            commands.entity(entity).despawn();
        }
        if experience.infinite_distance {
            experience.velocity *= 0.2_f32.powf(time.delta_seconds());
        } else {
            experience.velocity *= 0.025_f32.powf(time.delta_seconds());
        }
        transform.translation += experience.velocity * time.delta_seconds();
    }
}
