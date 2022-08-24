use self::{davy::DavyQuest, jagerossa::JagerossaQuest, plank::PlankQuest, ringo::RingoQuest};
use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

pub struct QuestsPlugin;

impl Plugin for QuestsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(jagerossa::JagerossaQuestPlugin)
            .add_plugin(davy::DavyQuestPlugin)
            .add_plugin(ringo::RingoQuestPlugin)
            .add_plugin(plank::PlankQuestPlugin)
            .add_system(quests_debug);
    }
}

#[derive(Default)]
pub struct Quests {
    pub active_quest: Quest,
}

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
        false
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

#[derive(Clone)]
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
            });
    });
}

pub mod davy;
pub mod jagerossa;
pub mod plank;
pub mod ringo;
