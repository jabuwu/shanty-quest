use std::time::Duration;

use bevy::asset::ChangeWatcher;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_egui::{egui, EguiContext};
use jam::common::prelude::*;
use jam::game::prelude::*;
use jam::DEV_BUILD;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Shanty Quest: Treble at Sea".to_string(),
                        resolution: WindowResolution::new(1280., 768.),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    watch_for_changes: if DEV_BUILD {
                        ChangeWatcher::with_delay(Duration::from_millis(500))
                    } else {
                        None
                    },
                    ..Default::default()
                }),
        )
        .add_plugins((
            jam::common::CommonPlugin,
            jam::loading::LoadingPlugin,
            jam::main_menu::MainMenuPlugin,
            jam::game::GamePlugin,
        ))
        .add_systems(Update, (stats, debug_dialogue))
        .run();
}

fn stats(
    mut egui_query: Query<&mut EguiContext>,
    mut menu_bar: ResMut<MenuBar>,
    state_time: Res<StateTime<AppState>>,
    game_state: Res<GameState>,
) {
    menu_bar.item("Stats", |open| {
        let Some(mut egui_context) = egui_query.get_single_mut().ok() else { return };
        egui::Window::new("Stats")
            .open(open)
            .show(egui_context.get_mut(), |ui| {
                ui.label(format!("State Time: {}", state_time.time));
                ui.label(format!("Game State: {:#?}", game_state));
            });
    });
}

fn debug_dialogue(
    mut egui_query: Query<&mut EguiContext>,
    mut menu_bar: ResMut<MenuBar>,
    mut dialogue: ResMut<Dialogue>,
) {
    menu_bar.item("Dialogues", |open| {
        let Some(mut egui_context) = egui_query.get_single_mut().ok() else { return };
        egui::Window::new("Dialogues")
            .open(open)
            .show(egui_context.get_mut(), |ui| {
                macro_rules! dialogue_button_for {
                    ($ui:ident, $e:expr) => {
                        if $ui.button(stringify!($e)).clicked() {
                            for (p, t) in $e.iter() {
                                dialogue.add_text(*p, String::from(*t));
                            }
                        }
                    };
                }
                ui.horizontal(|ui| {
                    dialogue_button_for!(ui, MUST_TALK_TO_MAYOR);
                    dialogue_button_for!(ui, MUST_TALK_TO_BARKEEP);
                    dialogue_button_for!(ui, JAGEROSSA1);
                    dialogue_button_for!(ui, JAGEROSSA2);
                    dialogue_button_for!(ui, DANGEROUS_SEAS);
                    dialogue_button_for!(ui, RINGO_MAYOR);
                });
                ui.horizontal(|ui| {
                    dialogue_button_for!(ui, RINGO1);
                    dialogue_button_for!(ui, RINGO2);
                    dialogue_button_for!(ui, PLANK_MAYOR);
                    dialogue_button_for!(ui, PLANK1);
                    dialogue_button_for!(ui, PLANK2);
                    dialogue_button_for!(ui, DAVY_MAYOR);
                });
                ui.horizontal(|ui| {
                    dialogue_button_for!(ui, DAVY1);
                    dialogue_button_for!(ui, DAVY2);
                    dialogue_button_for!(ui, MAYOR_RANDOM1);
                    dialogue_button_for!(ui, MAYOR_RANDOM2);
                    dialogue_button_for!(ui, MAYOR_RANDOM3);
                    dialogue_button_for!(ui, MAYOR_RANDOM4);
                });
                ui.horizontal(|ui| {
                    dialogue_button_for!(ui, MAYOR_RANDOM5);
                    dialogue_button_for!(ui, BARKEEP1);
                    dialogue_button_for!(ui, BARKEEP_RANDOM1);
                    dialogue_button_for!(ui, BARKEEP_RANDOM2);
                    dialogue_button_for!(ui, BARKEEP_RANDOM3);
                    dialogue_button_for!(ui, BARKEEP_RANDOM4);
                });
                ui.horizontal(|ui| {
                    dialogue_button_for!(ui, BARKEEP_RANDOM5);
                    dialogue_button_for!(ui, BARKEEP_RANDOM6);
                    dialogue_button_for!(ui, UPGRADE_MENU);
                    dialogue_button_for!(ui, JAGEROSSA_AFTER_VICTORY);
                    dialogue_button_for!(ui, MAYOR_AFTER_VICTORY1);
                });
                ui.horizontal(|ui| {
                    dialogue_button_for!(ui, MAYOR_AFTER_VICTORY2);
                    dialogue_button_for!(ui, MAYOR_AFTER_VICTORY3);
                    dialogue_button_for!(ui, MAYOR_AFTER_VICTORY4);
                    dialogue_button_for!(ui, MAYOR_AFTER_VICTORY5);
                    dialogue_button_for!(ui, MAYOR_AFTER_VICTORY6);
                });
            });
    });
}
