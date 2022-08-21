use audio_plus::prelude::*;
use bevy::prelude::*;
use examples_common::prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Audio Plus - Spawning".to_string(),
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
        .add_plugin(InstructionsPlugin("WASD to move".to_owned()))
        .add_startup_system(init)
        .add_system(spawn)
        .run();
}

fn init(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands
        .spawn_bundle(box_sprite(Vec2::ZERO, Color::GREEN))
        .insert(Player)
        .insert(AudioPlusListener);
}

#[derive(Default)]
struct SpawnData {
    spawn_time: f32,
}

fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut data: Local<SpawnData>,
    time: Res<Time>,
) {
    let time_to_spawn = 0.2;
    data.spawn_time += time.delta_seconds();
    if data.spawn_time >= time_to_spawn {
        let x = rand::random::<f32>() * 1000. - 500.;
        let y = rand::random::<f32>() * 600. - 300.;
        commands
            .spawn_bundle(box_sprite(Vec2::new(x, y), Color::BLUE))
            .insert(
                AudioPlusSource::new(
                    AudioPlusSoundEffect::single(asset_server.load("sounds/pong.ogg"))
                        .with_positional(true)
                        .with_pitch(1., 0.2),
                )
                .as_playing(),
            )
            .insert(TimeToLive(0.8));
        data.spawn_time = 0.;
    }
}
