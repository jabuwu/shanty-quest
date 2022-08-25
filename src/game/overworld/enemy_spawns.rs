use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;

pub struct EnemySpawnsPlugin;

impl Plugin for EnemySpawnsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DespawnSpawnedEntitiesEvent>()
            .add_system(enemy_spawns)
            .add_system(enemy_spawns_despawn);
    }
}

#[derive(Component, Default)]
pub struct SpawnedEntity {
    frames: u32,
}

#[derive(Default, Clone, Copy)]
pub struct DespawnSpawnedEntitiesEvent;

const DESPAWN_BUFFER_DISTANCE: f32 = 100.;
const RANDOM_SPAWN_DISTANCE: Vec2 = Vec2::new(1280. * 0.5 + 100., 768. * 0.5 + 100.);
fn random_spawn_offset() -> Vec2 {
    let area = rand::random::<u8>() % 4;
    if area == 0 {
        Vec2::new(
            RANDOM_SPAWN_DISTANCE.x,
            RANDOM_SPAWN_DISTANCE.y * rand::random::<f32>() * 2. - RANDOM_SPAWN_DISTANCE.y,
        )
    } else if area == 1 {
        Vec2::new(
            -RANDOM_SPAWN_DISTANCE.x,
            RANDOM_SPAWN_DISTANCE.y * rand::random::<f32>() * 2. - RANDOM_SPAWN_DISTANCE.y,
        )
    } else if area == 2 {
        Vec2::new(
            RANDOM_SPAWN_DISTANCE.x * rand::random::<f32>() * 2. - RANDOM_SPAWN_DISTANCE.x,
            RANDOM_SPAWN_DISTANCE.y,
        )
    } else {
        Vec2::new(
            RANDOM_SPAWN_DISTANCE.x * rand::random::<f32>() * 2. - RANDOM_SPAWN_DISTANCE.x,
            -RANDOM_SPAWN_DISTANCE.y,
        )
    }
}

fn enemy_spawns(
    mut commands: Commands,
    mut queries: ParamSet<(
        Query<&GlobalTransform, With<Player>>,
        Query<(Entity, &GlobalTransform, &mut SpawnedEntity)>,
    )>,
    mut ev_octopus_spawn: EventWriter<OctopusSpawnEvent>,
    state_time: Res<StateTime<AppState>>,
    game_state: Res<GameState>,
    screen_fade: Res<ScreenFade>,
    threat_level: Res<ThreatLevel>,
    cutscenes: Res<Cutscenes>,
    app_state: Res<State<AppState>>,
) {
    if cutscenes.running() && matches!(app_state.current(), AppState::Overworld) {
        return;
    }
    let player_position = if let Ok(player_transform) = queries.p0().get_single() {
        player_transform.translation().truncate()
    } else {
        Vec2::ZERO
    };
    let mut count = 0;
    for (entity, transform, mut spawned) in queries.p1().iter_mut() {
        spawned.frames += 1;
        let difference = (player_position - transform.translation().truncate()).abs();
        if spawned.frames > 10
            && (difference.x > RANDOM_SPAWN_DISTANCE.x + DESPAWN_BUFFER_DISTANCE
                || difference.y > RANDOM_SPAWN_DISTANCE.y + DESPAWN_BUFFER_DISTANCE)
        {
            commands.entity(entity).despawn_recursive();
        } else {
            count += 1;
        }
    }
    let spawn_chance = match *threat_level {
        ThreatLevel::None => 0.00,
        ThreatLevel::Easy => 0.02,
        ThreatLevel::Medium => 0.07,
        ThreatLevel::Hard => 0.1,
        ThreatLevel::Midnight => 0.20,
    };
    let spawn_max = match *threat_level {
        ThreatLevel::None => 0,
        ThreatLevel::Easy => 5,
        ThreatLevel::Medium => 10,
        ThreatLevel::Hard => 20,
        ThreatLevel::Midnight => 30,
    };
    if !state_time.just_entered()
        && rand::random::<f32>() < spawn_chance
        && count < spawn_max
        && !game_state.quests.block_enemy_spawns()
        && screen_fade.faded_in()
        && *threat_level != ThreatLevel::None
    {
        let entity = commands.spawn().insert(SpawnedEntity::default()).id();
        ev_octopus_spawn.send(OctopusSpawnEvent {
            entity: Some(entity),
            position: player_position + random_spawn_offset(),
        });
    }
}

fn enemy_spawns_despawn(
    mut ev_despawn: EventReader<DespawnSpawnedEntitiesEvent>,
    query: Query<(Entity, &SpawnedEntity)>,
    mut commands: Commands,
) {
    for _ in ev_despawn.iter() {
        for (entity, spawned) in query.iter() {
            if spawned.frames > 10 {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
