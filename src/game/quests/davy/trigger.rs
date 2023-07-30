use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;

use super::{Davy1Cutscene, DavyQuestStage};

pub struct DavyTriggerPlugin;

impl Plugin for DavyTriggerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (davy_trigger_world_spawn, davy_trigger_check));
    }
}

#[derive(Component)]
pub struct DavyTrigger;

fn davy_trigger_world_spawn(
    mut ev_spawn: EventReader<WorldLocationsSpawnEvent>,
    mut commands: Commands,
    world_locations: Res<WorldLocations>,
) {
    for _ in ev_spawn.iter() {
        let triggers = world_locations.get_multiple_rect("DavyTrigger");
        for trigger in triggers {
            commands.spawn((
                TransformBundle::default(),
                Transform2::from_translation(trigger.position).with_depth((DepthLayer::Entity, 0.)),
                Trigger::new(CollisionShape::Rect { size: trigger.size }),
                DavyTrigger,
            ));
        }
    }
}

fn davy_trigger_check(
    query: Query<&Trigger, With<DavyTrigger>>,
    mut game_state: ResMut<GameState>,
    mut ev_cutscene_davy1: EventWriter<CutsceneStartEvent<Davy1Cutscene>>,
) {
    for trigger in query.iter() {
        if trigger.triggered() {
            if let Quest::Davy(quest) = &mut game_state.quests.active_quest {
                if matches!(quest.stage, DavyQuestStage::TalkedToMayor) {
                    ev_cutscene_davy1.send_default();
                    quest.stage = DavyQuestStage::Dialogue1;
                }
            }
        }
    }
}
