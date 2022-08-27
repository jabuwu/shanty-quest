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

#[derive(Default, Clone, Debug)]
pub struct RingoQuest {
    pub stage: RingoQuestStage,
}

#[derive(Default, Clone, Debug)]
pub enum RingoQuestStage {
    #[default]
    TalkToMayor,
    TalkedToMayor,
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
        cutscene.add_quick_step(ringo1_cleanup);
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
    overworld_camera.arena_enable(rect.position, rect.size);
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
        cutscene.add_timed_step(|| {}, 2.5);
        cutscene.add_dialogue_step(ringo2_init1);
        cutscene.add_timed_step(ringo2_fade_out, 0.5);
        cutscene.add_quick_step(ringo2_cleanup);
    }
}

fn ringo2_init1(mut dialogue: ResMut<Dialogue>, mut game_state: ResMut<GameState>) {
    game_state.attacks.shockwave = 1;
    for (p, t) in RINGO2.iter() {
        dialogue.add_text(*p, String::from(*t));
    }
}

fn ringo2_fade_out(mut screen_fade: ResMut<ScreenFade>) {
    screen_fade.fade_out(0.5);
}

fn ringo2_cleanup(
    mut game_state: ResMut<GameState>,
    mut overworld_camera: ResMut<OverworldCamera>,
    mut app_state: ResMut<State<AppState>>,
    world_locations: Res<WorldLocations>,
) {
    overworld_camera.reset();
    game_state.quests.next();
    game_state.town = TownData::build("Isla de Dio", world_locations.as_ref());
    app_state.set(AppState::TownOutside).unwrap();
}

pub mod ringo;
pub mod trigger;
