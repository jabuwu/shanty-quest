use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;

use super::{Jagerossa1Cutscene, JagerossaQuestStage};

pub struct JagerossaTriggerPlugin;

impl Plugin for JagerossaTriggerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (jagerossa_trigger_world_spawn, jagerossa_trigger_check),
        );
    }
}

#[derive(Component)]
pub struct JagerossaTrigger;

fn jagerossa_trigger_world_spawn(
    mut ev_spawn: EventReader<WorldLocationsSpawnEvent>,
    mut commands: Commands,
    world_locations: Res<WorldLocations>,
) {
    for _ in ev_spawn.iter() {
        let triggers = world_locations.get_multiple_rect("JagerossaTrigger");
        for trigger in triggers {
            commands.spawn((
                TransformBundle::default(),
                Transform2::from_translation(trigger.position).with_depth((DepthLayer::Entity, 0.)),
                Trigger::new(CollisionShape::Rect { size: trigger.size }),
                JagerossaTrigger,
            ));
        }
    }
}

fn jagerossa_trigger_check(
    query: Query<&Trigger, With<JagerossaTrigger>>,
    mut game_state: ResMut<GameState>,
    mut ev_cutscene_jagerossa1: EventWriter<CutsceneStartEvent<Jagerossa1Cutscene>>,
) {
    for trigger in query.iter() {
        if trigger.triggered() {
            if let Quest::Jagerossa(quest) = &mut game_state.quests.active_quest {
                if matches!(quest.stage, JagerossaQuestStage::ControlsTutorial) {
                    ev_cutscene_jagerossa1.send_default();
                    quest.stage = JagerossaQuestStage::Dialogue1;
                }
            }
        }
    }
}
