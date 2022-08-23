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
            .add_cutscene::<Jagerossa2Cutscene>()
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
    Dialogue1,
    Fight,
    Dialogue2,
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
    mut overworld_camera: ResMut<OverworldCamera>,
    world_locations: Res<WorldLocations>,
) {
    ev_jagerossa_spawn.send_default();

    dialogue.add_text(
        "Ha-ha! Sailed right into me ambush ya bilge rat! I'll paint ya ship black with gunpowder!"
            .to_owned(),
    );
    dialogue.add_text("Then I'll take yer instrument from your scorched corpse!".to_owned());

    let rect = world_locations.get_single_rect("JagerossaArena");
    overworld_camera.enable_arena(rect.position, rect.size);
}

fn jagerossa1_cleanup(mut game_state: ResMut<GameState>) {
    if let Quest::Jagerossa(quest) = &mut game_state.quests.active_quest {
        quest.stage = JagerossaQuestStage::Fight;
    }
}

#[derive(Default, Debug, Clone)]
pub struct Jagerossa2Cutscene {
    pub boat: Option<Entity>,
    pub from: Vec2,
    pub to: Vec2,
}

impl Cutscene for Jagerossa2Cutscene {
    fn build(cutscene: &mut CutsceneBuilder) {
        cutscene.add_dialogue_step(jagerossa2_init1);
        cutscene.add_dialogue_step(jagerossa2_cleanup);
    }
}

fn jagerossa2_init1(mut dialogue: ResMut<Dialogue>) {
    dialogue.add_text(
        "Well! Ya can't always get what you want... But wait, don't kill me yet!".to_owned(),
    );
    dialogue.add_text("Have some sympathy fer me, poor devil... How about we combine our powers?! Ha?\nWith 2 instruments, yer ship we'll be unstoppable!".to_owned());
    dialogue.add_text("Other Pirate Lords will scatter like tumblin' dice before our combined might! Set sail, onwards! We need to find a town.".to_owned());
}

fn jagerossa2_cleanup(
    mut game_state: ResMut<GameState>,
    mut overworld_camera: ResMut<OverworldCamera>,
) {
    game_state.quests.next();
    overworld_camera.reset();
}

pub mod jagerossa;
pub mod trigger;
