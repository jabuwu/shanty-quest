use bevy::prelude::*;

fn main() {
    println!("{}", Vec2::new(1., 1.).normalize());
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Bevy Jam 2".to_string(),
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
        .run();
}
