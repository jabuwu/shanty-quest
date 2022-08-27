use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;

use super::{Jagerossa2Cutscene, JagerossaQuestStage};

pub struct JagerossaPlugin;

impl Plugin for JagerossaPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<JagerossaSpawnEvent>()
            .add_system(jagerossa_spawn)
            .add_system(jagerossa_move)
            .add_system(jagerossa_invincibility)
            .add_system(jagerossa_death_check);
    }
}

#[derive(Default, Clone, Copy)]
pub struct JagerossaSpawnEvent;

#[derive(Component)]
pub struct Jagerossa {
    target: Vec2,
    angle: f32,
    adjust_angle_chance: TimedChance,
    backoff_time: f32,
    backoff_dir: Vec2,
    backoff_chance: TimedChance,
}

struct JagerossaStatsByHealth {
    speed: f32,
    attack_time: f32,
}

fn jagerossa_stats_by_health(health_percent: f32) -> JagerossaStatsByHealth {
    if health_percent > 0.3 {
        JagerossaStatsByHealth {
            speed: 175.,
            attack_time: 1.,
        }
    } else {
        JagerossaStatsByHealth {
            speed: 300.,
            attack_time: 0.5,
        }
    }
}

fn jagerossa_spawn(
    mut ev_spawn: EventReader<JagerossaSpawnEvent>,
    mut ev_boat_spawn: EventWriter<BoatSpawnEvent>,
    mut commands: Commands,
    world_locations: Res<WorldLocations>,
    mut overworld_camera: ResMut<OverworldCamera>,
    mut ev_boss_healthbar_spawn: EventWriter<BossHealthbarSpawnEvent>,
) {
    let spawn_position = world_locations.get_single_position("JagerossaSpawn");
    for _ in ev_spawn.iter() {
        let stats = jagerossa_stats_by_health(1.);
        let entity = commands
            .spawn()
            .insert(Jagerossa {
                target: world_locations.get_single_position("JagerossaMoveTo"),
                angle: 0.,
                adjust_angle_chance: TimedChance::new(),
                backoff_time: 1.5,
                backoff_dir: Vec2::new(1., -1.),
                backoff_chance: TimedChance::new(),
            })
            .insert(Label("Jagerossa".to_owned()))
            .insert(AutoDamage {
                despawn: true,
                ..Default::default()
            })
            .id();
        overworld_camera.entity_focus(entity);
        ev_boss_healthbar_spawn.send(BossHealthbarSpawnEvent {
            name: "Captain Mike Jagerossa".to_owned(),
            entity,
        });
        ev_boat_spawn.send(BoatSpawnEvent {
            entity: Some(entity),
            position: spawn_position,
            attack: Attacks {
                shotgun_cannons: 1,
                ..Default::default()
            },
            healthbar: false,
            player: false,
            health: 40.,
            speed: stats.speed,
            attack_cooldown: stats.attack_time,
            knockback_resistance: 0.8,
        });
    }
}

fn jagerossa_move(
    mut queries: ParamSet<(
        Query<(&mut Boat, &GlobalTransform, &mut Jagerossa, &Health)>,
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
    for (mut boat, global_transform, mut jagerossa, health) in queries.p0().iter_mut() {
        let stats = jagerossa_stats_by_health(health.value / health.max);
        boat.speed = stats.speed;
        boat.shoot_cooldown_threshold = stats.attack_time;
        if cutscenes.running() {
            boat.movement = (jagerossa.target - global_transform.translation().truncate()) / 100.;
            if boat.movement.x.abs() < 0.1 {
                boat.movement.x = 0.
            }
            if boat.movement.y.abs() < 0.1 {
                boat.movement.y = 0.
            }
        } else {
            jagerossa.backoff_time -= time.delta_seconds();
            let mut destination = player_position + Vec2::from_angle(jagerossa.angle) * 200.;
            if jagerossa.backoff_time > 0. {
                destination = global_transform.translation().truncate()
                    + jagerossa.backoff_dir.normalize() * 300.;
            } else if jagerossa
                .backoff_chance
                .check(2.5, 0.25, time.delta_seconds())
            {
                jagerossa.backoff_dir =
                    (global_transform.translation().truncate() - player_position).normalize();
                if rand::random() {
                    jagerossa.backoff_dir = jagerossa.backoff_dir.perp();
                } else {
                    jagerossa.backoff_dir = -jagerossa.backoff_dir.perp();
                }
                jagerossa.backoff_time = 0.5;
            }
            let mut difference = destination - global_transform.translation().truncate();
            if difference.length() == 0. {
                difference = Vec2::ONE;
            }
            let applied_length = (difference.length() - 100.).max(0.) / 50.;
            if applied_length < 0.8
                || jagerossa
                    .adjust_angle_chance
                    .check(3., 3., time.delta_seconds())
            {
                jagerossa.angle += std::f32::consts::PI * 0.3;
            }
            boat.movement = difference.normalize() * applied_length;
        }
        if boat.movement.length_squared() > 0. {
            boat.direction = Vec2::X.angle_between(boat.movement);
        }
        boat.shoot = !cutscenes.running();
    }
}

fn jagerossa_invincibility(mut query: Query<(&mut Boat, &AutoDamage), With<Jagerossa>>) {
    for (mut boat, auto_damage) in query.iter_mut() {
        boat.opacity = if auto_damage.invincibility > 0. {
            0.5
        } else {
            1.
        };
    }
}

fn jagerossa_death_check(
    query: Query<Entity, With<Jagerossa>>,
    mut game_state: ResMut<GameState>,
    mut ev_cutscene_jagerossa2: EventWriter<CutsceneStartEvent<Jagerossa2Cutscene>>,
) {
    if query.is_empty() {
        if let Quest::Jagerossa(quest) = &mut game_state.quests.active_quest {
            if matches!(quest.stage, JagerossaQuestStage::Fight) {
                ev_cutscene_jagerossa2.send_default();
                quest.stage = JagerossaQuestStage::Dialogue2;
            }
        }
    }
}
