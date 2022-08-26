use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;

struct EnemySpawnsState {
    chance: TimedChance,
    none_level: EnemySpawnLevel,
    easy_level: EnemySpawnLevel,
    medium_level: EnemySpawnLevel,
    hard_level: EnemySpawnLevel,
    midnight_level: EnemySpawnLevel,
    davy_level: EnemySpawnLevel,
}

impl Default for EnemySpawnsState {
    fn default() -> Self {
        Self {
            chance: TimedChance::new(),
            none_level: EnemySpawnLevel {
                spawn_chances: vec![(1., EnemySpawn::Octopus(OctopusLevel::Easy))],
                seconds_per_spawn: 1.,
                spawn_max: 0,
            },
            easy_level: EnemySpawnLevel {
                spawn_chances: vec![
                    (0.03, EnemySpawn::Octopus(OctopusLevel::Medium)),
                    (1., EnemySpawn::Octopus(OctopusLevel::Easy)),
                ],
                seconds_per_spawn: 1.,
                spawn_max: 5,
            },
            medium_level: EnemySpawnLevel {
                spawn_chances: vec![
                    (0.05, EnemySpawn::Octopus(OctopusLevel::Hard)),
                    (0.1, EnemySpawn::Octopus(OctopusLevel::Medium)),
                    (1., EnemySpawn::Octopus(OctopusLevel::Easy)),
                ],
                seconds_per_spawn: 0.5,
                spawn_max: 10,
            },
            hard_level: EnemySpawnLevel {
                spawn_chances: vec![
                    (0.1, EnemySpawn::Octopus(OctopusLevel::Hard)),
                    (0.1, EnemySpawn::Octopus(OctopusLevel::Medium)),
                    (1., EnemySpawn::Octopus(OctopusLevel::Easy)),
                ],
                seconds_per_spawn: 0.25,
                spawn_max: 20,
            },
            midnight_level: EnemySpawnLevel {
                spawn_chances: vec![
                    (0.1, EnemySpawn::Octopus(OctopusLevel::Hard)),
                    (0.3, EnemySpawn::Octopus(OctopusLevel::Medium)),
                    (1., EnemySpawn::Octopus(OctopusLevel::Easy)),
                ],
                seconds_per_spawn: 0.1,
                spawn_max: 30,
            },
            davy_level: EnemySpawnLevel {
                spawn_chances: vec![(1., EnemySpawn::Octopus(OctopusLevel::Easy))],
                seconds_per_spawn: 1.0,
                spawn_max: 5,
            },
        }
    }
}

enum EnemySpawn {
    Octopus(OctopusLevel),
}

struct EnemySpawnLevel {
    spawn_chances: Vec<(f32, EnemySpawn)>,
    seconds_per_spawn: f32,
    spawn_max: i32,
}

pub struct EnemySpawnsPlugin;

impl Plugin for EnemySpawnsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnsState>()
            .add_event::<DespawnSpawnedEntitiesEvent>()
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
    mut state: ResMut<EnemySpawnsState>,
    time: Res<Time>,
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
    let EnemySpawnsState {
        chance,
        none_level,
        easy_level,
        medium_level,
        hard_level,
        midnight_level,
        davy_level,
    } = state.as_mut();
    let level = match *threat_level {
        ThreatLevel::None => none_level,
        ThreatLevel::Easy => easy_level,
        ThreatLevel::Medium => medium_level,
        ThreatLevel::Hard => hard_level,
        ThreatLevel::Midnight => midnight_level,
        ThreatLevel::Davy => davy_level,
    };
    if !state_time.just_entered()
        && chance.check(level.seconds_per_spawn, 0., time.delta_seconds())
        && count < level.spawn_max
        && !game_state.quests.block_enemy_spawns()
        && screen_fade.faded_in()
        && *threat_level != ThreatLevel::None
    {
        let position = player_position + random_spawn_offset();
        for spawn_chance in level.spawn_chances.iter() {
            if rand::random::<f32>() < spawn_chance.0 {
                match spawn_chance.1 {
                    EnemySpawn::Octopus(level) => {
                        let entity = commands.spawn().insert(SpawnedEntity::default()).id();
                        ev_octopus_spawn.send(OctopusSpawnEvent {
                            entity: Some(entity),
                            position,
                            level,
                        });
                    }
                }
                break;
            }
        }
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
