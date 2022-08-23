use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use jam::common::prelude::*;

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
