use self::{
    davy::{DavyQuest, DavyQuestStage},
    jagerossa::{JagerossaQuest, JagerossaQuestStage},
    plank::{PlankQuest, PlankQuestStage},
    ringo::{RingoQuest, RingoQuestStage},
};
use crate::game::prelude::*;
use crate::{common::prelude::*, DEV_BUILD};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use super::town::outside::rum_refill::RumRefillCutscene;

pub struct QuestsPlugin;

impl Plugin for QuestsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<QuestMayorEvent>()
            .add_event::<QuestBarkeepEvent>()
            .add_plugin(jagerossa::JagerossaQuestPlugin)
            .add_plugin(davy::DavyQuestPlugin)
            .add_plugin(ringo::RingoQuestPlugin)
            .add_plugin(plank::PlankQuestPlugin)
            .add_system(quests_debug)
            .add_system(quests_mayor)
            .add_system(quests_barkeep)
            .add_system(quests_skip);
    }
}

#[derive(Default, Debug, Clone)]
pub struct Quests {
    pub active_quest: Quest,
    pub mayor_dialogue: u32,
    pub mayor_after_dialogue: u32,
    pub barkeep_dialogue: u32,
    pub talked_to_barkeep: bool,
    pub endgame_town_dialogue: bool,
    pub upgrades_dialogue: bool,
}

#[derive(Default, Clone, Copy)]
pub struct QuestMayorEvent;

#[derive(Default, Clone, Copy)]
pub struct QuestBarkeepEvent;

impl Quests {
    pub fn hide_town_marker(&self) -> bool {
        self.block_town_enter()
    }

    pub fn block_town_enter(&self) -> bool {
        match &self.active_quest {
            Quest::Jagerossa(..) => true,
            Quest::Ringo(quest) => {
                matches!(quest.stage, RingoQuestStage::Dialogue1)
                    || matches!(quest.stage, RingoQuestStage::Fight)
                    || matches!(quest.stage, RingoQuestStage::Dialogue2)
            }
            Quest::Plank(quest) => {
                matches!(quest.stage, PlankQuestStage::Dialogue1)
                    || matches!(quest.stage, PlankQuestStage::Fight)
                    || matches!(quest.stage, PlankQuestStage::Dialogue2)
            }
            Quest::Davy(quest) => {
                matches!(quest.stage, DavyQuestStage::Dialogue1)
                    || matches!(quest.stage, DavyQuestStage::Fight)
                    || matches!(quest.stage, DavyQuestStage::Dialogue2)
            }
            _ => false,
        }
    }

    pub fn block_town_exit_cutscene(&self) -> bool {
        match self.active_quest {
            Quest::Jagerossa(..) => true,
            _ => false,
        }
    }

    pub fn block_enemy_spawns(&self) -> bool {
        if self.fighting() && !self.davy() {
            return true;
        }
        match &self.active_quest {
            Quest::Jagerossa(..) => true,
            _ => false,
        }
    }

    pub fn block_dangerous_seas(&self) -> bool {
        match &self.active_quest {
            Quest::Jagerossa(..) => true,
            Quest::Ringo(quest) => matches!(quest.stage, RingoQuestStage::TalkToMayor),
            _ => false,
        }
    }

    pub fn pirate_dialogue(&self) -> bool {
        match &self.active_quest {
            Quest::Jagerossa(quest) => {
                matches!(quest.stage, JagerossaQuestStage::Dialogue1)
                    || matches!(quest.stage, JagerossaQuestStage::Dialogue2)
            }
            Quest::Ringo(quest) => {
                matches!(quest.stage, RingoQuestStage::Dialogue1)
                    || matches!(quest.stage, RingoQuestStage::Dialogue2)
            }
            Quest::Plank(quest) => {
                matches!(quest.stage, PlankQuestStage::Dialogue1)
                    || matches!(quest.stage, PlankQuestStage::Dialogue2)
            }
            Quest::Davy(quest) => {
                matches!(quest.stage, DavyQuestStage::Dialogue1)
                    || matches!(quest.stage, DavyQuestStage::Dialogue2)
            }
            _ => false,
        }
    }

    pub fn fighting(&self) -> bool {
        match &self.active_quest {
            Quest::Jagerossa(quest) => matches!(quest.stage, JagerossaQuestStage::Fight),
            Quest::Ringo(quest) => matches!(quest.stage, RingoQuestStage::Fight),
            Quest::Plank(quest) => matches!(quest.stage, PlankQuestStage::Fight),
            Quest::Davy(quest) => matches!(quest.stage, DavyQuestStage::Fight),
            _ => false,
        }
    }

    pub fn must_talk_to_mayor(&self) -> bool {
        match &self.active_quest {
            Quest::Jagerossa(..) => false,
            Quest::Ringo(quest) => matches!(quest.stage, RingoQuestStage::TalkToMayor),
            Quest::Plank(quest) => matches!(quest.stage, PlankQuestStage::TalkToMayor),
            Quest::Davy(quest) => matches!(quest.stage, DavyQuestStage::TalkToMayor),
            _ => false,
        }
    }

    pub fn marker(&self) -> Option<&str> {
        if self.must_talk_to_mayor() || self.fighting() || self.pirate_dialogue() {
            return None;
        }
        match self.active_quest {
            Quest::Jagerossa(..) => None,
            Quest::Ringo(..) => Some("RingoTrigger"),
            Quest::Plank(..) => Some("PlankTrigger"),
            Quest::Davy(..) => Some("DavyTrigger"),
            Quest::End => None,
        }
    }

    pub fn objective(&self) -> Option<(f32, &str)> {
        if self.jagerossa() {
            return None;
        }
        if self.ringo() && self.must_talk_to_mayor() {
            return None;
        }
        if self.fighting() || self.pirate_dialogue() {
            return None;
        }
        if self.must_talk_to_mayor() {
            return Some((264., "Talk to the governor at town"));
        }
        match self.active_quest {
            Quest::Jagerossa(..) => Some((284., "Defeat Captain Mike Jagerossa")),
            Quest::Ringo(..) => Some((240., "Defeat Captain Ringo Yarr")),
            Quest::Plank(..) => Some((268., "Defeat Captain Plank Presley")),
            Quest::Davy(..) => Some((244., "Defeat Captain Davy Bowie")),
            Quest::End => None,
        }
    }

    pub fn jagerossa(&self) -> bool {
        match &self.active_quest {
            Quest::Jagerossa(..) => true,
            _ => false,
        }
    }

    pub fn ringo(&self) -> bool {
        match &self.active_quest {
            Quest::Ringo(..) => true,
            _ => false,
        }
    }

    pub fn plank(&self) -> bool {
        match &self.active_quest {
            Quest::Plank(..) => true,
            _ => false,
        }
    }

    pub fn davy(&self) -> bool {
        match &self.active_quest {
            Quest::Davy(..) => true,
            _ => false,
        }
    }

    pub fn end(&self) -> bool {
        match &self.active_quest {
            Quest::End => true,
            _ => false,
        }
    }

    pub fn next(&mut self) {
        self.active_quest.next();
    }
}

#[derive(Clone, Debug)]
pub enum Quest {
    Jagerossa(JagerossaQuest),
    Ringo(RingoQuest),
    Plank(PlankQuest),
    Davy(DavyQuest),
    End,
}

impl Quest {
    pub fn next(&mut self) {
        *self = match *self {
            Self::Jagerossa(..) => Self::Ringo(RingoQuest::default()),
            Self::Ringo(..) => Self::Plank(PlankQuest::default()),
            Self::Plank(..) => Self::Davy(DavyQuest::default()),
            Self::Davy(..) => Self::End,
            Self::End => Self::End,
        }
    }
}

impl Default for Quest {
    fn default() -> Self {
        Self::Jagerossa(JagerossaQuest::default())
        //Self::End
    }
}

fn quests_mayor(
    mut ev_mayor: EventReader<QuestMayorEvent>,
    mut dialogue: ResMut<Dialogue>,
    mut game_state: ResMut<GameState>,
) {
    let mut fallback_dialogue = true;
    for _ in ev_mayor.iter() {
        match &mut game_state.quests.active_quest {
            Quest::Ringo(quest) => {
                if matches!(quest.stage, RingoQuestStage::TalkToMayor) {
                    for (p, t) in RINGO_MAYOR.iter() {
                        dialogue.add_text(*p, String::from(*t));
                    }
                    quest.stage = RingoQuestStage::TalkedToMayor;
                    fallback_dialogue = false;
                }
            }
            Quest::Plank(quest) => {
                if matches!(quest.stage, PlankQuestStage::TalkToMayor) {
                    for (p, t) in PLANK_MAYOR.iter() {
                        dialogue.add_text(*p, String::from(*t));
                    }
                    quest.stage = PlankQuestStage::TalkedToMayor;
                    fallback_dialogue = false;
                }
            }
            Quest::Davy(quest) => {
                if matches!(quest.stage, DavyQuestStage::TalkToMayor) {
                    for (p, t) in DAVY_MAYOR.iter() {
                        dialogue.add_text(*p, String::from(*t));
                    }
                    quest.stage = DavyQuestStage::TalkedToMayor;
                    fallback_dialogue = false;
                }
            }
            _ => {}
        }
        if fallback_dialogue {
            if game_state.quests.end() {
                match game_state.quests.mayor_after_dialogue % 6 {
                    0 => {
                        for (p, t) in MAYOR_AFTER_VICTORY1.iter() {
                            dialogue.add_text(*p, String::from(*t));
                        }
                    }
                    1 => {
                        for (p, t) in MAYOR_AFTER_VICTORY2.iter() {
                            dialogue.add_text(*p, String::from(*t));
                        }
                    }
                    2 => {
                        for (p, t) in MAYOR_AFTER_VICTORY3.iter() {
                            dialogue.add_text(*p, String::from(*t));
                        }
                    }
                    3 => {
                        for (p, t) in MAYOR_AFTER_VICTORY4.iter() {
                            dialogue.add_text(*p, String::from(*t));
                        }
                    }
                    4 => {
                        for (p, t) in MAYOR_AFTER_VICTORY5.iter() {
                            dialogue.add_text(*p, String::from(*t));
                        }
                    }
                    _ => {
                        for (p, t) in MAYOR_AFTER_VICTORY6.iter() {
                            dialogue.add_text(*p, String::from(*t));
                        }
                    }
                }
                game_state.quests.mayor_after_dialogue =
                    (game_state.quests.mayor_after_dialogue + 1) % 6;
            } else {
                match game_state.quests.mayor_dialogue % 5 {
                    0 => {
                        for (p, t) in MAYOR_RANDOM1.iter() {
                            dialogue.add_text(*p, String::from(*t));
                        }
                    }
                    1 => {
                        for (p, t) in MAYOR_RANDOM2.iter() {
                            dialogue.add_text(*p, String::from(*t));
                        }
                    }
                    2 => {
                        for (p, t) in MAYOR_RANDOM3.iter() {
                            dialogue.add_text(*p, String::from(*t));
                        }
                    }
                    3 => {
                        for (p, t) in MAYOR_RANDOM4.iter() {
                            dialogue.add_text(*p, String::from(*t));
                        }
                    }
                    _ => {
                        for (p, t) in MAYOR_RANDOM5.iter() {
                            dialogue.add_text(*p, String::from(*t));
                        }
                    }
                }
                game_state.quests.mayor_dialogue = (game_state.quests.mayor_dialogue + 1) % 5;
            }
        }
    }
}

fn quests_barkeep(
    mut ev_barkeep: EventReader<QuestBarkeepEvent>,
    mut dialogue: ResMut<Dialogue>,
    mut game_state: ResMut<GameState>,
    mut ev_rum_refill_cutscene: EventWriter<CutsceneStartEvent<RumRefillCutscene>>,
) {
    let mut fallback_dialogue = true;
    for _ in ev_barkeep.iter() {
        let need_rum = game_state.health != game_state.health_max;
        if !game_state.quests.talked_to_barkeep {
            game_state.quests.talked_to_barkeep = true;
            for (p, t) in BARKEEP1.iter() {
                dialogue.add_text(*p, String::from(*t));
            }
            fallback_dialogue = false;
        }
        if fallback_dialogue && !need_rum {
            match game_state.quests.barkeep_dialogue % 6 {
                0 => {
                    for (p, t) in BARKEEP_RANDOM1.iter() {
                        dialogue.add_text(*p, String::from(*t));
                    }
                }
                1 => {
                    for (p, t) in BARKEEP_RANDOM2.iter() {
                        dialogue.add_text(*p, String::from(*t));
                    }
                }
                2 => {
                    for (p, t) in BARKEEP_RANDOM3.iter() {
                        dialogue.add_text(*p, String::from(*t));
                    }
                }
                3 => {
                    for (p, t) in BARKEEP_RANDOM4.iter() {
                        dialogue.add_text(*p, String::from(*t));
                    }
                }
                4 => {
                    for (p, t) in BARKEEP_RANDOM5.iter() {
                        dialogue.add_text(*p, String::from(*t));
                    }
                }
                _ => {
                    for (p, t) in BARKEEP_RANDOM6.iter() {
                        dialogue.add_text(*p, String::from(*t));
                    }
                }
            }
            game_state.quests.barkeep_dialogue = (game_state.quests.barkeep_dialogue + 1) % 6;
        }
        if need_rum {
            ev_rum_refill_cutscene.send_default();
        }
    }
}

fn quests_debug(
    mut egui_context: ResMut<EguiContext>,
    mut menu_bar: ResMut<MenuBar>,
    game_state: Res<GameState>,
) {
    menu_bar.item("Quest", |open| {
        egui::Window::new("Quest")
            .open(open)
            .show(egui_context.ctx_mut(), |ui| {
                ui.label(format!(
                    "Active Quest: {}",
                    match game_state.quests.active_quest {
                        Quest::Jagerossa(..) => "Jagerossa",
                        Quest::Ringo(..) => "Ringo",
                        Quest::Plank(..) => "Plank",
                        Quest::Davy(..) => "Davy",
                        Quest::End => "End",
                    }
                ));
                ui.label(format!("{:?}", game_state.quests));
            });
    });
}

pub fn quests_skip(
    input: Res<Input<KeyCode>>,
    mut game_state: ResMut<GameState>,
    mut player_query: Query<&mut Transform2, With<Player>>,
    app_state: Res<State<AppState>>,
) {
    if !DEV_BUILD {
        return;
    }
    if input.just_pressed(KeyCode::P) {
        game_state.skill_points += 1;
    }
    if *app_state.current() != AppState::Overworld {
        return;
    }
    if input.just_pressed(KeyCode::F2) {
        game_state.quests.active_quest = Quest::End;
        game_state.quests.talked_to_barkeep = true;
        game_state.dangerous_seas = true;
    }
    if input.just_pressed(KeyCode::Key7) {
        game_state.quests.active_quest = Quest::Ringo(RingoQuest::default());
        if let Quest::Ringo(quest) = &mut game_state.quests.active_quest {
            quest.stage = RingoQuestStage::TalkedToMayor;
        }
        game_state.quests.talked_to_barkeep = true;
        game_state.dangerous_seas = true;
        game_state.attacks.shotgun_cannons = 1;
        game_state.skill_points = 99;
        if let Ok(mut transform) = player_query.get_single_mut() {
            transform.translation = Vec2::new(7000., -1250.);
        };
    }
    if input.just_pressed(KeyCode::Key8) {
        game_state.quests.active_quest = Quest::Plank(PlankQuest::default());
        if let Quest::Plank(quest) = &mut game_state.quests.active_quest {
            quest.stage = PlankQuestStage::TalkedToMayor;
        }
        game_state.quests.talked_to_barkeep = true;
        game_state.dangerous_seas = true;
        game_state.attacks.shotgun_cannons = 1;
        game_state.attacks.shockwave = 1;
        game_state.skill_points = 99;
        if let Ok(mut transform) = player_query.get_single_mut() {
            transform.translation = Vec2::new(7500., -6750.);
        };
    }
    if input.just_pressed(KeyCode::Key9) {
        game_state.quests.active_quest = Quest::Davy(DavyQuest::default());
        if let Quest::Davy(quest) = &mut game_state.quests.active_quest {
            quest.stage = DavyQuestStage::TalkedToMayor;
        }
        game_state.quests.talked_to_barkeep = true;
        game_state.dangerous_seas = true;
        game_state.attacks.shotgun_cannons = 1;
        game_state.attacks.shockwave = 1;
        game_state.attacks.bombs = 1;
        game_state.skill_points = 99;
        if let Ok(mut transform) = player_query.get_single_mut() {
            transform.translation = Vec2::new(2850., -9650.);
        };
    }
}

pub mod davy;
pub mod jagerossa;
pub mod plank;
pub mod ringo;
