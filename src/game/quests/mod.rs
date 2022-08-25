use self::{
    davy::{DavyQuest, DavyQuestStage},
    jagerossa::JagerossaQuest,
    plank::{PlankQuest, PlankQuestStage},
    ringo::{RingoQuest, RingoQuestStage},
};
use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

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
            .add_system(quests_barkeep);
    }
}

#[derive(Default, Debug)]
pub struct Quests {
    pub active_quest: Quest,
    pub mayor_dialogue: u32,
    pub barkeep_dialogue: u32,
    pub talked_to_barkeep: bool,
}

#[derive(Default, Clone, Copy)]
pub struct QuestMayorEvent;

#[derive(Default, Clone, Copy)]
pub struct QuestBarkeepEvent;

impl Quests {
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
        match self.active_quest {
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
        if self.must_talk_to_mayor() {
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

    pub fn objective(&self) -> Option<&str> {
        if self.must_talk_to_mayor() {
            return Some("Talk to mayor");
        }
        match self.active_quest {
            Quest::Jagerossa(..) => Some("Defeat Jagerossa"),
            Quest::Ringo(..) => Some("Defeat Ringo"),
            Quest::Plank(..) => Some("Defeat Plank"),
            Quest::Davy(..) => Some("Defeat Davy"),
            Quest::End => None,
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
        //Self::Jagerossa(JagerossaQuest::default())
        Self::End
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

fn quests_barkeep(
    mut ev_barkeep: EventReader<QuestBarkeepEvent>,
    mut dialogue: ResMut<Dialogue>,
    mut game_state: ResMut<GameState>,
) {
    let mut fallback_dialogue = true;
    for _ in ev_barkeep.iter() {
        if !game_state.quests.talked_to_barkeep {
            game_state.quests.talked_to_barkeep = true;
            for (p, t) in BARKEEP1.iter() {
                dialogue.add_text(*p, String::from(*t));
            }
            fallback_dialogue = false;
        }
        if fallback_dialogue {
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

pub mod davy;
pub mod jagerossa;
pub mod plank;
pub mod ringo;
