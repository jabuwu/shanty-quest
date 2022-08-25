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
}

fn jagerossa_spawn(
    mut ev_spawn: EventReader<JagerossaSpawnEvent>,
    mut ev_boat_spawn: EventWriter<BoatSpawnEvent>,
    mut commands: Commands,
    world_locations: Res<WorldLocations>,
) {
    let spawn_position = world_locations.get_single_position("JagerossaSpawn");
    for _ in ev_spawn.iter() {
        let entity = commands
            .spawn()
            .insert(Jagerossa {
                target: world_locations.get_single_position("JagerossaMoveTo"),
            })
            .insert(Label("Jagerossa".to_owned()))
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

fn jagerossa_move(mut query: Query<(&mut Boat, &GlobalTransform, &Jagerossa)>) {
    for (mut boat, global_transform, jagerossa) in query.iter_mut() {
        boat.movement = (jagerossa.target - global_transform.translation().truncate()) / 100.;
        if boat.movement.x.abs() < 0.1 {
            boat.movement.x = 0.
        }
        if boat.movement.y.abs() < 0.1 {
            boat.movement.y = 0.
        }
        if boat.movement.length_squared() > 0. {
            boat.direction = Vec2::X.angle_between(boat.movement);
        }
        /*if rand::random::<f32>() < 0.05 {
            boat.special_shoot = true;
        }*/
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
