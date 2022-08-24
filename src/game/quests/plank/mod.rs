use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;

use self::plank::PlankSpawnEvent;

pub struct PlankQuestPlugin;

impl Plugin for PlankQuestPlugin {
    fn build(&self, app: &mut App) {
        app.add_cutscene::<Plank1Cutscene>()
            .add_cutscene::<Plank2Cutscene>()
            .add_plugin(plank::PlankPlugin)
            .add_plugin(trigger::PlankTriggerPlugin);
    }
}

#[derive(Default, Clone, Debug)]
pub struct PlankQuest {
    stage: PlankQuestStage,
}

#[derive(Default, Clone, Debug)]
enum PlankQuestStage {
    #[default]
    ControlsTutorial,
    Dialogue1,
    Fight,
    Dialogue2,
}

#[derive(Default, Debug, Clone)]
pub struct Plank1Cutscene {
    pub boat: Option<Entity>,
    pub from: Vec2,
    pub to: Vec2,
}

impl Cutscene for Plank1Cutscene {
    fn build(cutscene: &mut CutsceneBuilder) {
        cutscene.add_dialogue_step(plank1_init1);
        cutscene.add_dialogue_step(plank1_cleanup);
    }
}

fn plank1_init1(
    mut dialogue: ResMut<Dialogue>,
    mut ev_plank_spawn: EventWriter<PlankSpawnEvent>,
    mut overworld_camera: ResMut<OverworldCamera>,
    world_locations: Res<WorldLocations>,
) {
    ev_plank_spawn.send_default();

    for (p, t) in PLANK1.iter() {
        dialogue.add_text(*p, String::from(*t));
    }

    let rect = world_locations.get_single_rect("PlankArena");
    overworld_camera.enable_arena(rect.position, rect.size);
}

fn plank1_cleanup(mut game_state: ResMut<GameState>) {
    if let Quest::Plank(quest) = &mut game_state.quests.active_quest {
        quest.stage = PlankQuestStage::Fight;
    }
}

#[derive(Default, Debug, Clone)]
pub struct Plank2Cutscene {
    pub boat: Option<Entity>,
    pub from: Vec2,
    pub to: Vec2,
}

impl Cutscene for Plank2Cutscene {
    fn build(cutscene: &mut CutsceneBuilder) {
        cutscene.add_dialogue_step(plank2_init1);
        cutscene.add_dialogue_step(plank2_cleanup);
    }
}

fn plank2_init1(mut dialogue: ResMut<Dialogue>) {
    for (p, t) in PLANK2.iter() {
        dialogue.add_text(*p, String::from(*t));
    }
}

fn plank2_cleanup(
    mut game_state: ResMut<GameState>,
    mut overworld_camera: ResMut<OverworldCamera>,
) {
    game_state.quests.next();
    overworld_camera.reset();
}

pub mod plank;
pub mod trigger;
