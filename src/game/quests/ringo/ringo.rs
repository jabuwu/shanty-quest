use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;

use super::{Ringo2Cutscene, RingoQuestStage};

pub struct RingoPlugin;

impl Plugin for RingoPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RingoSpawnEvent>()
            .add_system(ringo_spawn)
            .add_system(ringo_move)
            .add_system(ringo_invincibility)
            .add_system(ringo_death_check);
    }
}

#[derive(Default, Clone, Copy)]
pub struct RingoSpawnEvent;

#[derive(Component)]
pub struct Ringo {
    target: Vec2,
    angle: f32,
    dash_chance: TimedChance,
    charge_time: f32,
}

fn ringo_spawn(
    mut ev_spawn: EventReader<RingoSpawnEvent>,
    mut ev_boat_spawn: EventWriter<BoatSpawnEvent>,
    mut commands: Commands,
    world_locations: Res<WorldLocations>,
    mut ev_enemies_despawn: EventWriter<DespawnSpawnedEntitiesEvent>,
    mut overworld_camera: ResMut<OverworldCamera>,
) {
    let spawn_position = world_locations.get_single_position("RingoSpawn");
    for _ in ev_spawn.iter() {
        ev_enemies_despawn.send_default();
        let entity = commands
            .spawn()
            .insert(Ringo {
                target: world_locations.get_single_position("RingoMoveTo"),
                angle: 0.,
                dash_chance: TimedChance::new(),
                charge_time: 0.,
            })
            .insert(Label("Ringo".to_owned()))
            .insert(AutoDamage {
                despawn: true,
                ..Default::default()
            })
            .id();
        overworld_camera.entity_focus(entity);
        ev_boat_spawn.send(BoatSpawnEvent {
            entity: Some(entity),
            position: spawn_position,
            attack: Attacks {
                shockwave: 1,
                ..Default::default()
            },
            healthbar: true,
            player: false,
            health: 30.,
            speed: 175.,
            attack_cooldown: 1.,
            knockback_resistance: 0.8,
        });
    }
}

fn ringo_move(
    mut queries: ParamSet<(
        Query<(&mut Boat, &GlobalTransform, &mut Ringo)>,
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
    for (mut boat, global_transform, mut ringo) in queries.p0().iter_mut() {
        boat.shoot = false;
        if cutscenes.running() {
            boat.movement = (ringo.target - global_transform.translation().truncate()) / 100.;
            if boat.movement.x.abs() < 0.1 {
                boat.movement.x = 0.
            }
            if boat.movement.y.abs() < 0.1 {
                boat.movement.y = 0.
            }
        } else {
            if ringo.charge_time < 0. && ringo.dash_chance.check(2., 1., time.delta_seconds()) {
                ringo.charge_time = 1.;
            }
            boat.shoot = global_transform
                .translation()
                .truncate()
                .distance(player_position)
                < 250.;
            let destination = if ringo.charge_time > -1. {
                player_position
            } else {
                player_position + Vec2::from_angle(ringo.angle) * 300.
            };
            let last_dash_time = ringo.charge_time;
            ringo.charge_time -= time.delta_seconds();
            if ringo.charge_time <= 0. && last_dash_time > 0. {
                boat.dash = true;
            }
            let mut difference = destination - global_transform.translation().truncate();
            if difference.length() == 0. {
                difference = Vec2::ONE;
            }
            let mut applied_length = (difference.length() - 100.).max(0.) / 50.;
            if applied_length < 0.8 {
                ringo.angle += std::f32::consts::PI * 0.3;
            }
            if ringo.charge_time > 0. {
                applied_length = 0.05;
            }
            boat.movement = difference.normalize() * applied_length;
        }
        if boat.movement.length_squared() > 0. {
            boat.direction = Vec2::X.angle_between(boat.movement);
        }
    }
}

fn ringo_invincibility(mut query: Query<(&mut Boat, &AutoDamage), With<Ringo>>) {
    for (mut boat, auto_damage) in query.iter_mut() {
        boat.opacity = if auto_damage.invincibility > 0. {
            0.5
        } else {
            1.
        };
    }
}

fn ringo_death_check(
    query: Query<Entity, With<Ringo>>,
    mut game_state: ResMut<GameState>,
    mut ev_cutscene_ringo2: EventWriter<CutsceneStartEvent<Ringo2Cutscene>>,
) {
    if query.is_empty() {
        if let Quest::Ringo(quest) = &mut game_state.quests.active_quest {
            if matches!(quest.stage, RingoQuestStage::Fight) {
                ev_cutscene_ringo2.send_default();
                quest.stage = RingoQuestStage::Dialogue2;
            }
        }
    }
}
