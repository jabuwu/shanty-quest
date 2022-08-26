use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;

use super::{Davy2Cutscene, DavyQuestStage};

pub struct DavyPlugin;

impl Plugin for DavyPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DavySpawnEvent>()
            .add_system(davy_spawn)
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
}

fn davy_spawn(
    mut ev_spawn: EventReader<DavySpawnEvent>,
    mut ev_boat_spawn: EventWriter<BoatSpawnEvent>,
    mut commands: Commands,
    world_locations: Res<WorldLocations>,
    mut ev_enemies_despawn: EventWriter<DespawnSpawnedEntitiesEvent>,
) {
    let spawn_position = world_locations.get_single_position("DavySpawn");
    for _ in ev_spawn.iter() {
        ev_enemies_despawn.send_default();
        let entity = commands
            .spawn()
            .insert(Davy {
                target: world_locations.get_single_position("DavyMoveTo"),
            })
            .insert(Label("Davy".to_owned()))
            .insert(AutoDamage {
                despawn: true,
                ..Default::default()
            })
            .id();
        ev_boat_spawn.send(BoatSpawnEvent {
            entity: Some(entity),
            position: spawn_position,
            special_attack: Attacks {
                kraken: 1,
                ..Default::default()
            },
            healthbar: true,
            player: false,
            health: 30.,
            speed: 100.,
        });
    }
}

fn davy_move(mut query: Query<(&mut Boat, &GlobalTransform, &Davy)>) {
    for (mut boat, global_transform, davy) in query.iter_mut() {
        boat.movement = (davy.target - global_transform.translation().truncate()) / 100.;
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
