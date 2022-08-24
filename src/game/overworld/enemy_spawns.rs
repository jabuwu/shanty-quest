use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;

pub struct EnemySpawnsPlugin;

impl Plugin for EnemySpawnsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(AppState::Overworld).with_system(enemy_spawns));
    }
}

#[derive(Component)]
pub struct SpawnedEntity;

const RANDOM_SPAWN_DISTANCE: Vec2 = Vec2::new(1000., 700.);
fn random_spawn_offset() -> Vec2 {
    let area = rand::random::<u8>() % 4;
    if area == 0 {
        Vec2::new(
            RANDOM_SPAWN_DISTANCE.x,
            RANDOM_SPAWN_DISTANCE.y * rand::random::<f32>(),
        )
    } else if area == 1 {
        Vec2::new(
            -RANDOM_SPAWN_DISTANCE.x,
            RANDOM_SPAWN_DISTANCE.y * rand::random::<f32>(),
        )
    } else if area == 2 {
        Vec2::new(
            RANDOM_SPAWN_DISTANCE.x * rand::random::<f32>(),
            RANDOM_SPAWN_DISTANCE.y,
        )
    } else {
        Vec2::new(
            RANDOM_SPAWN_DISTANCE.x * rand::random::<f32>(),
            -RANDOM_SPAWN_DISTANCE.y,
        )
    }
}

fn enemy_spawns(
    mut commands: Commands,
    mut queries: ParamSet<(
        Query<&GlobalTransform, With<Player>>,
        Query<(Entity, &GlobalTransform), With<SpawnedEntity>>,
    )>,
    mut ev_octopus_spawn: EventWriter<OctopusSpawnEvent>,
    state_time: Res<StateTime<AppState>>,
    game_state: Res<GameState>,
) {
    let player_position = if let Ok(player_transform) = queries.p0().get_single() {
        player_transform.translation().truncate()
    } else {
        Vec2::ZERO
    };
    let mut count = 0;
    for (entity, transform) in queries.p1().iter() {
        if transform.translation().truncate().distance(player_position) > 2000. {
            commands.entity(entity).despawn_recursive();
        } else {
            count += 1;
        }
    }
    if !state_time.just_entered()
        && rand::random::<f32>() < 0.02
        && count < 30
        && !game_state.quests.block_enemy_spawns()
    {
        let entity = commands.spawn().insert(SpawnedEntity).id();
        ev_octopus_spawn.send(OctopusSpawnEvent {
            entity: Some(entity),
            position: player_position + random_spawn_offset(),
        });
    }
}
