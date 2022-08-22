use bevy::prelude::*;
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
        .add_startup_system(screen_fade_enable)
        .run();
}

pub fn screen_fade_enable(mut screen_fade: ResMut<ScreenFade>) {
    screen_fade.enable();
}
