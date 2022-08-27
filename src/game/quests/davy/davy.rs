use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;

use super::{Davy2Cutscene, DavyQuestStage};

pub struct DavyPlugin;

impl Plugin for DavyPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DavySpawnEvent>()
            .add_system(davy_spawn.before(BoatSystems::Spawn))
            .add_system(davy_move)
            .add_system(davy_invincibility)
            .add_system(davy_death_check);
    }
}

#[derive(Default, Clone, Copy)]
pub struct DavySpawnEvent;

#[derive(Component)]
pub struct Davy {
    target: Vec2,
    angle: f32,
    adjust_angle_chance: TimedChance,
}

struct DavyStatsByHealth {
    speed: f32,
    attack_time: f32,
}

fn davy_stats_by_health(health_percent: f32) -> DavyStatsByHealth {
    if health_percent > 0.5 {
        DavyStatsByHealth {
            speed: 150.,
            attack_time: 0.75,
        }
    } else if health_percent > 0.15 {
        DavyStatsByHealth {
            speed: 250.,
            attack_time: 0.25,
        }
    } else {
        DavyStatsByHealth {
            speed: 400.,
            attack_time: 0.2,
        }
    }
}

fn davy_spawn(
    mut ev_spawn: EventReader<DavySpawnEvent>,
    mut ev_boat_spawn: EventWriter<BoatSpawnEvent>,
    mut commands: Commands,
    world_locations: Res<WorldLocations>,
    mut ev_enemies_despawn: EventWriter<DespawnSpawnedEntitiesEvent>,
    mut overworld_camera: ResMut<OverworldCamera>,
    mut ev_boss_healthbar_spawn: EventWriter<BossHealthbarSpawnEvent>,
) {
    let spawn_position = world_locations.get_single_position("DavySpawn");
    for _ in ev_spawn.iter() {
        let stats = davy_stats_by_health(1.);
        ev_enemies_despawn.send_default();
        let entity = commands
            .spawn()
            .insert(Davy {
                target: world_locations.get_single_position("DavyMoveTo"),
                angle: 0.,
                adjust_angle_chance: TimedChance::new(),
            })
            .insert(AutoDamage {
                despawn: true,
                ..Default::default()
            })
            .id();
        overworld_camera.entity_focus(entity);
        ev_boss_healthbar_spawn.send(BossHealthbarSpawnEvent {
            name: "Captain Davy Bowie".to_owned(),
            entity,
        });
        ev_boat_spawn.send(BoatSpawnEvent {
            entity: Some(entity),
            position: spawn_position,
            attack: Attacks {
                kraken: 1,
                ..Default::default()
            },
            healthbar: false,
            player: false,
            health: 100.,
            speed: stats.speed,
            attack_cooldown: stats.attack_time,
            knockback_resistance: 1.0,
        });
    }
}

fn davy_move(
    mut queries: ParamSet<(
        Query<(&mut Boat, &GlobalTransform, &mut Davy, &Health)>,
        Query<&GlobalTransform, With<Player>>,
    )>,
    cutscenes: Res<Cutscenes>,
    time: Res<Time>,
) {
    let player_position = if let Ok(player_transform) = queries.p1().get_single() {
        player_transform.translation().truncate()
    } else {
        Vec2::ZERO
    };
    for (mut boat, global_transform, mut davy, health) in queries.p0().iter_mut() {
        let stats = davy_stats_by_health(health.value / health.max);
        boat.speed = stats.speed;
        boat.shoot_cooldown_threshold = stats.attack_time;
        if cutscenes.running() {
            boat.movement = (davy.target - global_transform.translation().truncate()) / 100.;
            if boat.movement.x.abs() < 0.1 {
                boat.movement.x = 0.
            }
            if boat.movement.y.abs() < 0.1 {
                boat.movement.y = 0.
            }
        } else {
            let destination = player_position + Vec2::from_angle(davy.angle) * 100.;
            let mut difference = destination - global_transform.translation().truncate();
            if difference.length() == 0. {
                difference = Vec2::ONE;
            }
            let applied_length = (difference.length() - 100.).max(0.) / 50.;
            if applied_length < 0.8 || davy.adjust_angle_chance.check(3., 3., time.delta_seconds())
            {
                davy.angle += std::f32::consts::PI * 0.3;
            }
            boat.movement = difference.normalize() * applied_length;
        }
        if boat.movement.length_squared() > 0. {
            boat.direction = Vec2::X.angle_between(boat.movement);
        }
        boat.shoot = !cutscenes.running();
    }
}

fn davy_invincibility(mut query: Query<(&mut Boat, &AutoDamage), With<Davy>>) {
    for (mut boat, auto_damage) in query.iter_mut() {
        boat.opacity = if auto_damage.invincibility > 0. {
            0.5
        } else {
            1.
        };
    }
}

fn davy_death_check(
    query: Query<Entity, With<Davy>>,
    mut game_state: ResMut<GameState>,
    mut ev_cutscene_davy2: EventWriter<CutsceneStartEvent<Davy2Cutscene>>,
) {
    if query.is_empty() {
        if let Quest::Davy(quest) = &mut game_state.quests.active_quest {
            if matches!(quest.stage, DavyQuestStage::Fight) {
                ev_cutscene_davy2.send_default();
                quest.stage = DavyQuestStage::Dialogue2;
            }
        }
    }
}
