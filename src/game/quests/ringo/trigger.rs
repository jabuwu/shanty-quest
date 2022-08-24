use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;

use super::{Ringo1Cutscene, RingoQuestStage};

pub struct RingoTriggerPlugin;

impl Plugin for RingoTriggerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(ringo_trigger_world_spawn)
            .add_system(ringo_trigger_check);
    }
}

#[derive(Component)]
pub struct RingoTrigger;

fn ringo_trigger_world_spawn(
    mut ev_spawn: EventReader<WorldLocationsSpawnEvent>,
    mut commands: Commands,
    world_locations: Res<WorldLocations>,
) {
    for _ in ev_spawn.iter() {
        let triggers = world_locations.get_multiple_rect("RingoTrigger");
        for trigger in triggers {
            commands
                .spawn_bundle(TransformBundle::default())
                .insert(
                    Transform2::from_translation(trigger.position)
                        .with_depth((DepthLayer::Entity, 0.)),
                )
                .insert(Trigger::new(CollisionShape::Rect { size: trigger.size }))
                .insert(Label("Ringo Trigger".to_owned()))
                .insert(RingoTrigger);
        }
    }
}

fn ringo_trigger_check(
    query: Query<&Trigger, With<RingoTrigger>>,
    mut game_state: ResMut<GameState>,
    mut ev_cutscene_ringo1: EventWriter<CutsceneStartEvent<Ringo1Cutscene>>,
) {
    for trigger in query.iter() {
        if trigger.triggered() {
            if let Quest::Ringo(quest) = &mut game_state.quests.active_quest {
                if matches!(quest.stage, RingoQuestStage::TalkedToMayor) {
                    ev_cutscene_ringo1.send_default();
                    quest.stage = RingoQuestStage::Dialogue1;
                }
            }
        }
    }
}
