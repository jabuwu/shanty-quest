use audio_plus::prelude::*;
use bevy::prelude::*;
use examples_common::prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Audio Plus - Fade".to_string(),
            width: 1280.,
            height: 720.,
            resizable: false,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlusPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(TimeToLivePlugin)
        .add_plugin(InstructionsPlugin(
            "Press SPACE to play sound\nPress S to stop sound".to_owned(),
        ))
        .add_startup_system(init)
        .add_system(controls)
        .run();
}

fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn().insert(AudioPlusSource::new(
        AudioPlusSoundEffect::single(asset_server.load("sounds/music_1.ogg")).with_fade(3., 3.),
    ));
}

fn controls(input: Res<Input<KeyCode>>, mut query: Query<&mut AudioPlusSource>) {
    if input.just_pressed(KeyCode::Space) {
        for mut source in query.iter_mut() {
            source.play_looped();
        }
    }
    if input.just_pressed(KeyCode::S) {
        for mut source in query.iter_mut() {
            source.stop();
        }
    }
}
