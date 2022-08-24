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
}

fn ringo_spawn(
    mut ev_spawn: EventReader<RingoSpawnEvent>,
    mut ev_boat_spawn: EventWriter<BoatSpawnEvent>,
    mut commands: Commands,
    player_query: Query<&GlobalTransform, With<Player>>,
    world_locations: Res<WorldLocations>,
) {
    let spawn_position = if let Ok(player_transform) = player_query.get_single() {
        player_transform.translation().truncate() + Vec2::new(810., -150.)
    } else {
        Vec2::ZERO
    };
    for _ in ev_spawn.iter() {
        let entity = commands
            .spawn()
            .insert(Ringo {
                target: world_locations.get_single_position("RingoMoveTo"),
            })
            .insert(Label("Ringo".to_owned()))
            .insert(AutoDamage {
                despawn: true,
                ..Default::default()
            })
            .id();
        ev_boat_spawn.send(BoatSpawnEvent {
            entity: Some(entity),
            position: spawn_position,
            special_attack: SpecialAttack::ShotgunCannons,
            healthbar: true,
            player: false,
        });
    }
}

fn ringo_move(mut query: Query<(&mut Boat, &GlobalTransform, &Ringo)>) {
    for (mut boat, global_transform, ringo) in query.iter_mut() {
        boat.movement = (ringo.target - global_transform.translation().truncate()) / 100.;
        if boat.movement.x.abs() < 0.1 {
            boat.movement.x = 0.
        }
        if boat.movement.y.abs() < 0.1 {
            boat.movement.y = 0.
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
