use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;

use self::ringo::RingoSpawnEvent;

pub struct RingoQuestPlugin;

impl Plugin for RingoQuestPlugin {
    fn build(&self, app: &mut App) {
        app.add_cutscene::<Ringo1Cutscene>()
            .add_cutscene::<Ringo2Cutscene>()
            .add_plugin(ringo::RingoPlugin)
            .add_plugin(trigger::RingoTriggerPlugin);
    }
}

#[derive(Default, Clone)]
pub struct RingoQuest {
    stage: RingoQuestStage,
}

#[derive(Default, Clone)]
enum RingoQuestStage {
    #[default]
    ControlsTutorial,
    Dialogue1,
    Fight,
    Dialogue2,
}

#[derive(Default, Debug, Clone)]
pub struct Ringo1Cutscene {
    pub boat: Option<Entity>,
    pub from: Vec2,
    pub to: Vec2,
}

impl Cutscene for Ringo1Cutscene {
    fn build(cutscene: &mut CutsceneBuilder) {
        cutscene.add_dialogue_step(ringo1_init1);
        cutscene.add_dialogue_step(ringo1_cleanup);
    }
}

fn ringo1_init1(
    mut dialogue: ResMut<Dialogue>,
    mut ev_ring_spawn: EventWriter<RingoSpawnEvent>,
    mut overworld_camera: ResMut<OverworldCamera>,
    world_locations: Res<WorldLocations>,
) {
    ev_ring_spawn.send_default();

    for (p, t) in RINGO1.iter() {
        dialogue.add_text(*p, String::from(*t));
    }

    let rect = world_locations.get_single_rect("RingoArena");
    overworld_camera.enable_arena(rect.position, rect.size);
}

fn ringo1_cleanup(mut game_state: ResMut<GameState>) {
    if let Quest::Ringo(quest) = &mut game_state.quests.active_quest {
        quest.stage = RingoQuestStage::Fight;
    }
}

#[derive(Default, Debug, Clone)]
pub struct Ringo2Cutscene {
    pub boat: Option<Entity>,
    pub from: Vec2,
    pub to: Vec2,
}

impl Cutscene for Ringo2Cutscene {
    fn build(cutscene: &mut CutsceneBuilder) {
        cutscene.add_dialogue_step(ringo2_init1);
        cutscene.add_dialogue_step(ringo2_cleanup);
    }
}

fn ringo2_init1(mut dialogue: ResMut<Dialogue>) {
    for (p, t) in RINGO2.iter() {
        dialogue.add_text(*p, String::from(*t));
    }
}

fn ringo2_cleanup(
    mut game_state: ResMut<GameState>,
    mut overworld_camera: ResMut<OverworldCamera>,
) {
    game_state.quests.next();
    overworld_camera.reset();
}

pub mod ringo;
pub mod trigger;
