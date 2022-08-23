use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;
use jagerossa::JagerossaSpawnEvent;

#[derive(Default)]
pub enum JagerossaQuestState {
    #[default]
    Empty,
}

pub struct JagerossaQuestPlugin;

impl Plugin for JagerossaQuestPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<JagerossaQuestState>()
            .add_cutscene::<Jagerossa1Cutscene>()
            .add_plugin(jagerossa::JagerossaPlugin)
            .add_plugin(trigger::JagerossaTriggerPlugin);
    }
}

#[derive(Default, Clone)]
pub struct JagerossaQuest {
    stage: JagerossaQuestStage,
}

#[derive(Default, Clone)]
enum JagerossaQuestStage {
    #[default]
    ControlsTutorial,
    Fight,
}

#[derive(Default, Debug, Clone)]
pub struct Jagerossa1Cutscene {
    pub boat: Option<Entity>,
    pub from: Vec2,
    pub to: Vec2,
}

impl Cutscene for Jagerossa1Cutscene {
    fn build(cutscene: &mut CutsceneBuilder) {
        cutscene.add_dialogue_step(jagerossa1_init1);
        cutscene.add_dialogue_step(jagerossa1_cleanup);
    }
}

fn jagerossa1_init1(
    mut dialogue: ResMut<Dialogue>,
    mut ev_jagerossa_spawn: EventWriter<JagerossaSpawnEvent>,
) {
    ev_jagerossa_spawn.send_default();

    dialogue.add_text(
        "Ha-ha! Sailed right into me ambush ya bilge rat! I'll paint ya ship black with gunpowder!"
            .to_owned(),
    );
    dialogue.add_text("Then I'll take yer instrument from your scorched corpse!".to_owned());
}

fn jagerossa1_cleanup(mut game_state: ResMut<GameState>) {
    game_state.quests.next();
}

pub mod jagerossa;
pub mod trigger;
