use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;

use self::davy::DavySpawnEvent;

pub struct DavyQuestPlugin;

impl Plugin for DavyQuestPlugin {
    fn build(&self, app: &mut App) {
        app.add_cutscene::<Davy1Cutscene>()
            .add_cutscene::<Davy2Cutscene>()
            .add_plugin(davy::DavyPlugin)
            .add_plugin(trigger::DavyTriggerPlugin);
    }
}

#[derive(Default, Clone)]
pub struct DavyQuest {
    stage: DavyQuestStage,
}

#[derive(Default, Clone)]
enum DavyQuestStage {
    #[default]
    ControlsTutorial,
    Dialogue1,
    Fight,
    Dialogue2,
}

#[derive(Default, Debug, Clone)]
pub struct Davy1Cutscene {
    pub boat: Option<Entity>,
    pub from: Vec2,
    pub to: Vec2,
}

impl Cutscene for Davy1Cutscene {
    fn build(cutscene: &mut CutsceneBuilder) {
        cutscene.add_dialogue_step(davy1_init1);
        cutscene.add_dialogue_step(davy1_cleanup);
    }
}

fn davy1_init1(
    mut dialogue: ResMut<Dialogue>,
    mut ev_davy_spawn: EventWriter<DavySpawnEvent>,
    mut overworld_camera: ResMut<OverworldCamera>,
    world_locations: Res<WorldLocations>,
) {
    ev_davy_spawn.send_default();

    for (p, t) in DAVY1.iter() {
        dialogue.add_text(*p, String::from(*t));
    }

    let rect = world_locations.get_single_rect("DavyArena");
    overworld_camera.enable_arena(rect.position, rect.size);
}

fn davy1_cleanup(mut game_state: ResMut<GameState>) {
    if let Quest::Davy(quest) = &mut game_state.quests.active_quest {
        quest.stage = DavyQuestStage::Fight;
    }
}

#[derive(Default, Debug, Clone)]
pub struct Davy2Cutscene {
    pub boat: Option<Entity>,
    pub from: Vec2,
    pub to: Vec2,
}

impl Cutscene for Davy2Cutscene {
    fn build(cutscene: &mut CutsceneBuilder) {
        cutscene.add_dialogue_step(davy2_init1);
        cutscene.add_dialogue_step(davy2_cleanup);
    }
}

fn davy2_init1(mut dialogue: ResMut<Dialogue>) {
    for (p, t) in DAVY2.iter() {
        dialogue.add_text(*p, String::from(*t));
    }
}

fn davy2_cleanup(mut game_state: ResMut<GameState>, mut overworld_camera: ResMut<OverworldCamera>) {
    game_state.quests.next();
    overworld_camera.reset();
}

pub mod davy;
pub mod trigger;
