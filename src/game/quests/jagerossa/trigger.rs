use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;

use super::{Jagerossa1Cutscene, JagerossaQuestStage};

pub struct JagerossaTriggerPlugin;

impl Plugin for JagerossaTriggerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(jagerossa_trigger_spawn)
            .add_system(jagerossa_trigger_check);
    }
}

#[derive(Component)]
pub struct JagerossaTrigger;

fn jagerossa_trigger_spawn(
    mut ev_overworld_enter: EventReader<OverworldEnterEvent>,
    mut commands: Commands,
) {
    for _ in ev_overworld_enter.iter() {
        commands
            .spawn_bundle(TransformBundle::default())
            .insert(Transform2::from_xy(602., -560.).with_depth((DepthLayer::Entity, 0.)))
            .insert(Trigger::new(CollisionShape::Rect {
                size: Vec2::new(384., 384.),
            }))
            .insert(Label("Jagerossa Trigger".to_owned()))
            .insert(JagerossaTrigger);
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
                    quest.stage = JagerossaQuestStage::Fight;
                }
            }
        }
    }
}
