use self::jagerossa::JagerossaQuest;
use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

pub struct QuestsPlugin;

impl Plugin for QuestsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(jagerossa::JagerossaQuestPlugin)
            .add_system(quests_debug);
    }
}

#[derive(Default)]
pub struct Quests {
    pub active_quest: Quest,
}

impl Quests {
    pub fn block_town_enter(&self) -> bool {
        false
    }

    pub fn block_town_exit_cutscene(&self) -> bool {
        match self.active_quest {
            Quest::Jagerossa(..) => true,
            _ => false,
        }
    }

    pub fn next(&mut self) {
        self.active_quest.next();
    }
}

#[derive(Clone)]
pub enum Quest {
    Jagerossa(JagerossaQuest),
    End,
}

impl Quest {
    pub fn next(&mut self) {
        *self = match *self {
            Self::Jagerossa(..) => Self::End,
            _ => Self::End,
        }
    }
}

impl Default for Quest {
    fn default() -> Self {
        Self::Jagerossa(JagerossaQuest::default())
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
                        Quest::End => "End",
                    }
                ));
            });
    });
}

pub mod jagerossa;
