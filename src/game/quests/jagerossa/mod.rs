use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;
use jagerossa::JagerossaSpawnEvent;

pub struct JagerossaQuestPlugin;

impl Plugin for JagerossaQuestPlugin {
    fn build(&self, app: &mut App) {
        app.add_cutscene::<Jagerossa1Cutscene>()
            .add_cutscene::<Jagerossa2Cutscene>()
            .add_plugin(jagerossa::JagerossaPlugin)
            .add_plugin(trigger::JagerossaTriggerPlugin);
    }
}

#[derive(Default, Clone, Debug)]
pub struct JagerossaQuest {
    pub stage: JagerossaQuestStage,
}

#[derive(Default, Clone, Debug)]
pub enum JagerossaQuestStage {
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

    for (p, t) in JAGEROSSA1.iter() {
        dialogue.add_text(*p, String::from(*t));
    }

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

fn jagerossa2_init1(mut dialogue: ResMut<Dialogue>, mut game_state: ResMut<GameState>) {
    game_state.attacks.shotgun_cannons = 1;
    for (p, t) in JAGEROSSA2.iter() {
        dialogue.add_text(*p, String::from(*t));
    }
}

fn jagerossa2_cleanup(
    mut game_state: ResMut<GameState>,
    mut overworld_camera: ResMut<OverworldCamera>,
    player_query: Query<Entity, With<Player>>,
    mut commands: Commands,
    world_locations: Res<WorldLocations>,
) {
    if let Ok(player_entity) = player_query.get_single() {
        commands
            .entity(player_entity)
            .insert(CharacterControllerDestination {
                target: world_locations.get_single_position("Portallica") + Vec2::new(0., -100.),
            });
    }
    game_state.quests.next();
    overworld_camera.reset();
}

pub mod jagerossa;
pub mod trigger;
