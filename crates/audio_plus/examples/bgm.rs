use audio_plus::prelude::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Audio Plus - BGM".to_string(),
            width: 1280.,
            height: 720.,
            resizable: false,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlusPlugin)
        .add_startup_system(init)
        .run();
}

fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn()
        .insert(AudioPlusSource::new(asset_server.load("sounds/music_1.ogg").into()).as_looping());
}
