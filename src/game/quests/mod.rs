use self::{
    davy::DavyQuest,
    jagerossa::JagerossaQuest,
    plank::PlankQuest,
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
}

#[derive(Default, Clone, Copy)]
pub struct QuestMayorEvent;

#[derive(Default, Clone, Copy)]
pub struct QuestBarkeepEvent;

impl Quests {
    pub fn block_town_enter(&self) -> bool {
        match self.active_quest {
            Quest::Jagerossa(..) => true,
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
        /*match self.active_quest {
            Quest::Jagerossa(..) => true,
            _ => false,
        }*/
        true
    }

    pub fn block_dangerous_seas(&self) -> bool {
        match self.active_quest {
            Quest::Jagerossa(..) => true,
            _ => false,
        }
    }

    pub fn must_talk_to_mayor(&self) -> bool {
        match &self.active_quest {
            Quest::Jagerossa(..) => false,
            Quest::Ringo(quest) => matches!(quest.stage, RingoQuestStage::TalkToMayor),
            _ => false,
        }
    }

    pub fn marker(&self) -> Option<&str> {
        match self.active_quest {
            Quest::Jagerossa(..) => None,
            Quest::Ringo(..) => Some("RingoTrigger"),
            Quest::Plank(..) => Some("PlankTrigger"),
            Quest::Davy(..) => Some("DavyTrigger"),
            Quest::End => None,
        }
    }

    pub fn objective(&self) -> Option<&str> {
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
            _ => {}
        }
        if fallback_dialogue {
            dialogue.add_text(DialoguePortrait::Mayor, "yippity yappity".to_string());
        }
    }
}

fn quests_barkeep(mut ev_barkeep: EventReader<QuestBarkeepEvent>, mut dialogue: ResMut<Dialogue>) {
    for _ in ev_barkeep.iter() {
        dialogue.add_text(DialoguePortrait::Barkeep, "babaa".to_string());
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
