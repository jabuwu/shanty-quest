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
            special_attack: SpecialAttack {
                bombs: 1,
                ..Default::default()
            },
            healthbar: true,
            player: false,
        });
    }
}

fn plank_move(mut query: Query<(&mut Boat, &GlobalTransform, &Plank)>) {
    for (mut boat, global_transform, plank) in query.iter_mut() {
        boat.movement = (plank.target - global_transform.translation().truncate()) / 100.;
        if boat.movement.x.abs() < 0.1 {
            boat.movement.x = 0.
        }
        if boat.movement.y.abs() < 0.1 {
            boat.movement.y = 0.
        }
        if boat.movement.length_squared() > 0. {
            boat.direction = Vec2::X.angle_between(boat.movement);
        }
        if rand::random::<f32>() < 0.05 {
            boat.shoot = true;
        }
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
