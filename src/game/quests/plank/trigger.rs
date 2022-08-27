use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;

use super::{Plank1Cutscene, PlankQuestStage};

pub struct PlankTriggerPlugin;

impl Plugin for PlankTriggerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(plank_trigger_world_spawn)
            .add_system(plank_trigger_check);
    }
}

#[derive(Component)]
pub struct PlankTrigger;

fn plank_trigger_world_spawn(
    mut ev_spawn: EventReader<WorldLocationsSpawnEvent>,
    mut commands: Commands,
    world_locations: Res<WorldLocations>,
) {
    for _ in ev_spawn.iter() {
        let triggers = world_locations.get_multiple_rect("PlankTrigger");
        for trigger in triggers {
            commands
                .spawn_bundle(TransformBundle::default())
                .insert(
                    Transform2::from_translation(trigger.position)
                        .with_depth((DepthLayer::Entity, 0.)),
                )
                .insert(Trigger::new(CollisionShape::Rect { size: trigger.size }))
                .insert(PlankTrigger);
        }
    }
}

fn plank_trigger_check(
    query: Query<&Trigger, With<PlankTrigger>>,
    mut game_state: ResMut<GameState>,
    mut ev_cutscene_plank1: EventWriter<CutsceneStartEvent<Plank1Cutscene>>,
) {
    for trigger in query.iter() {
        if trigger.triggered() {
            if let Quest::Plank(quest) = &mut game_state.quests.active_quest {
                if matches!(quest.stage, PlankQuestStage::TalkedToMayor) {
                    ev_cutscene_plank1.send_default();
                    quest.stage = PlankQuestStage::Dialogue1;
                }
            }
        }
    }
}
