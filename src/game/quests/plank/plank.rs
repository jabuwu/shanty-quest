use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;

use super::{Plank2Cutscene, PlankQuestStage};

pub struct PlankPlugin;

impl Plugin for PlankPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlankSpawnEvent>()
            .add_system(plank_spawn)
            .add_system(plank_move)
            .add_system(plank_invincibility)
            .add_system(plank_death_check);
    }
}

#[derive(Default, Clone, Copy)]
pub struct PlankSpawnEvent;

#[derive(Component)]
pub struct Plank {
    target: Vec2,
    angle: f32,
    adjust_angle_chance: TimedChance,
    backoff_time: f32,
    backoff_dir: Vec2,
    backoff_chance: TimedChance,
    backoff_stop: bool,
}

fn plank_spawn(
    mut ev_spawn: EventReader<PlankSpawnEvent>,
    mut ev_boat_spawn: EventWriter<BoatSpawnEvent>,
    mut commands: Commands,
    world_locations: Res<WorldLocations>,
    mut ev_enemies_despawn: EventWriter<DespawnSpawnedEntitiesEvent>,
) {
    let spawn_position = world_locations.get_single_position("PlankSpawn");
    for _ in ev_spawn.iter() {
        ev_enemies_despawn.send_default();
        let entity = commands
            .spawn()
            .insert(Plank {
                target: world_locations.get_single_position("PlankMoveTo"),
                angle: 0.,
                adjust_angle_chance: TimedChance::new(),
                backoff_time: 0.5,
                backoff_dir: Vec2::new(-1., 0.),
                backoff_chance: TimedChance::new(),
                backoff_stop: false,
            })
            .insert(Label("Plank".to_owned()))
            .insert(AutoDamage {
                despawn: true,
                ..Default::default()
            })
            .id();
        ev_boat_spawn.send(BoatSpawnEvent {
            entity: Some(entity),
            position: spawn_position,
            attack: Attacks {
                bombs: 1,
                ..Default::default()
            },
            healthbar: true,
            player: false,
            health: 30.,
            speed: 400.,
            attack_cooldown: 0.3,
            knockback_resistance: 0.3,
        });
    }
}

fn plank_move(
    mut queries: ParamSet<(
        Query<(&mut Boat, &GlobalTransform, &mut Plank)>,
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
    for (mut boat, global_transform, mut plank) in queries.p0().iter_mut() {
        if cutscenes.running() {
            boat.movement = (plank.target - global_transform.translation().truncate()) / 100.;
            if boat.movement.x.abs() < 0.1 {
                boat.movement.x = 0.
            }
            if boat.movement.y.abs() < 0.1 {
                boat.movement.y = 0.
            }
        } else {
            plank.backoff_time -= time.delta_seconds();
            let mut destination = player_position + Vec2::from_angle(plank.angle) * 200.;
            if plank.backoff_time > 0. {
                if plank.backoff_stop {
                    destination = global_transform.translation().truncate();
                } else {
                    destination = global_transform.translation().truncate()
                        + plank.backoff_dir.normalize() * 300.;
                }
            } else if plank.backoff_chance.check(2.5, 0.25, time.delta_seconds()) {
                plank.backoff_dir =
                    (global_transform.translation().truncate() - player_position).normalize();
                if rand::random() {
                    plank.backoff_dir = plank.backoff_dir.perp();
                } else {
                    plank.backoff_dir = -plank.backoff_dir.perp();
                }
                plank.backoff_stop = rand::random();
                plank.backoff_time = 0.5;
            }
            let mut difference = destination - global_transform.translation().truncate();
            if difference.length() == 0. {
                difference = Vec2::ONE;
            }
            let applied_length = (difference.length() - 100.).max(0.) / 50.;
            if applied_length < 0.8
                || plank
                    .adjust_angle_chance
                    .check(3., 3., time.delta_seconds())
            {
                plank.angle += std::f32::consts::PI * 0.3;
            }
            boat.movement = difference.normalize() * applied_length;
        }
        if boat.movement.length_squared() > 0. {
            boat.direction = Vec2::X.angle_between(boat.movement);
        }
        boat.shoot = !cutscenes.running();
    }
}

fn plank_invincibility(mut query: Query<(&mut Boat, &AutoDamage), With<Plank>>) {
    for (mut boat, auto_damage) in query.iter_mut() {
        boat.opacity = if auto_damage.invincibility > 0. {
            0.5
        } else {
            1.
        };
    }
}

fn plank_death_check(
    query: Query<Entity, With<Plank>>,
    mut game_state: ResMut<GameState>,
    mut ev_cutscene_plank2: EventWriter<CutsceneStartEvent<Plank2Cutscene>>,
) {
    if query.is_empty() {
        if let Quest::Plank(quest) = &mut game_state.quests.active_quest {
            if matches!(quest.stage, PlankQuestStage::Fight) {
                ev_cutscene_plank2.send_default();
                quest.stage = PlankQuestStage::Dialogue2;
            }
        }
    }
}
