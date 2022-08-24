use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use jam::common::prelude::*;
use jam::game::prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Shanty Quest: Treble at Sea".to_string(),
            width: 1280.,
            height: 768.,
            resizable: false,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins)
        .add_plugin(jam::common::CommonPlugin)
        .add_plugin(jam::loading::LoadingPlugin)
        .add_plugin(jam::main_menu::MainMenuPlugin)
        .add_plugin(jam::game::GamePlugin)
        .add_system(stats)
        .add_system(debug_dialogue)
        .run();
}

fn stats(
    mut egui_context: ResMut<EguiContext>,
    mut menu_bar: ResMut<MenuBar>,
    state_time: Res<StateTime<AppState>>,
) {
    menu_bar.item("Stats", |open| {
        egui::Window::new("Stats")
            .open(open)
            .show(egui_context.ctx_mut(), |ui| {
                ui.label(format!("State Time: {}", state_time.time));
            });
    });
}

fn debug_dialogue(
    mut egui_context: ResMut<EguiContext>,
    mut menu_bar: ResMut<MenuBar>,
    mut dialogue: ResMut<Dialogue>,
) {
    menu_bar.item("Dialogues", |open| {
        egui::Window::new("Dialogues")
            .open(open)
            .show(egui_context.ctx_mut(), |ui| {
                macro_rules! dialogue_button_for {
                    ($e:expr) => {
                        if ui.button(stringify!($e)).clicked() {
                            for (p, t) in $e.iter() {
                                dialogue.add_text(*p, String::from(*t));
                            }
                        }
                    };
                }
                dialogue_button_for!(MUST_TALK_TO_MAYOR);
                dialogue_button_for!(JAGEROSSA1);
                dialogue_button_for!(JAGEROSSA2);
                dialogue_button_for!(DANGEROUS_SEAS);
                dialogue_button_for!(RINGO_MAYOR);
                dialogue_button_for!(RINGO1);
                dialogue_button_for!(RINGO2);
                dialogue_button_for!(PLANK_MAYOR);
                dialogue_button_for!(PLANK1);
                dialogue_button_for!(PLANK2);
                dialogue_button_for!(DAVY_MAYOR);
                dialogue_button_for!(DAVY1);
                dialogue_button_for!(DAVY2);
                dialogue_button_for!(MAYOR_RANDOM1);
                dialogue_button_for!(MAYOR_RANDOM2);
                dialogue_button_for!(MAYOR_RANDOM3);
                dialogue_button_for!(MAYOR_RANDOM4);
                dialogue_button_for!(MAYOR_RANDOM5);
                dialogue_button_for!(BARKEEP1);
                dialogue_button_for!(BARKEEP_RANDOM1);
                dialogue_button_for!(BARKEEP_RANDOM2);
                dialogue_button_for!(BARKEEP_RANDOM3);
                dialogue_button_for!(BARKEEP_RANDOM4);
                dialogue_button_for!(BARKEEP_RANDOM5);
                dialogue_button_for!(BARKEEP_RANDOM6);
            });
    });
}
