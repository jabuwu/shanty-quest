use audio_plus::prelude::*;
use bevy::prelude::*;
use examples_common::prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Audio Plus - Random".to_string(),
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
            "Press SPACE to play a random sound".to_owned(),
        ))
        .add_startup_system(init)
        .add_system(controls)
        .run();
}

fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
    let sounds = AudioPlusSoundEffect::multiple(vec![
        asset_server.load("sounds/rock.ogg"),
        asset_server.load("sounds/paper.ogg"),
        asset_server.load("sounds/scissors.ogg"),
    ])
    .with_voices(3)
    .with_pitch(1., 0.2)
    .with_volume(0.5, 0.5);
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn().insert(AudioPlusSource::new(sounds));
}

fn controls(input: Res<Input<KeyCode>>, mut query: Query<&mut AudioPlusSource>) {
    if input.just_pressed(KeyCode::Space) {
        for mut source in query.iter_mut() {
            source.play();
        }
    }
}
