use audio_plus::prelude::*;
use bevy::prelude::*;
use examples_common::prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Audio Plus - Mixer".to_string(),
            width: 1280.,
            height: 720.,
            resizable: false,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlusPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(InstructionsPlugin(
            "WASD to move\nPress F to toggle all SFX\nPress M to toggle all music".to_owned(),
        ))
        .add_startup_system(init)
        .add_system(controls)
        .run();
}

fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands
        .spawn_bundle(box_sprite(Vec2::ZERO, Color::GREEN))
        .insert(Player)
        .insert(AudioPlusListener);
    commands
        .spawn_bundle(box_sprite(Vec2::new(300., 0.), Color::BLUE))
        .insert(
            AudioPlusSource::new(
                AudioPlusSoundEffect::single(asset_server.load("sounds/music_1.ogg"))
                    .with_positional(true)
                    .with_distance(350.)
                    .with_channel(AudioPlusMixerChannel::Music),
            )
            .as_looping(),
        );
    commands
        .spawn_bundle(box_sprite(Vec2::new(-300., 0.), Color::BLUE))
        .insert(
            AudioPlusSource::new(
                AudioPlusSoundEffect::single(asset_server.load("sounds/music_2.ogg"))
                    .with_positional(true)
                    .with_distance(350.)
                    .with_channel(AudioPlusMixerChannel::Music),
            )
            .as_looping(),
        );
    commands
        .spawn_bundle(box_sprite(Vec2::new(0., -200.), Color::ORANGE))
        .insert(
            AudioPlusSource::new(
                AudioPlusSoundEffect::single(asset_server.load("sounds/rock.ogg"))
                    .with_positional(true)
                    .with_distance(150.)
                    .with_channel(AudioPlusMixerChannel::Sfx),
            )
            .as_looping(),
        );
    commands
        .spawn_bundle(box_sprite(Vec2::new(200., -100.), Color::ORANGE))
        .insert(
            AudioPlusSource::new(
                AudioPlusSoundEffect::single(asset_server.load("sounds/scissors.ogg"))
                    .with_positional(true)
                    .with_distance(150.)
                    .with_channel(AudioPlusMixerChannel::Sfx),
            )
            .as_looping(),
        );
    commands
        .spawn_bundle(box_sprite(Vec2::new(-100., 100.), Color::ORANGE))
        .insert(
            AudioPlusSource::new(
                AudioPlusSoundEffect::single(asset_server.load("sounds/paper.ogg"))
                    .with_positional(true)
                    .with_distance(150.)
                    .with_channel(AudioPlusMixerChannel::Sfx),
            )
            .as_looping(),
        );
}

fn controls(input: Res<Input<KeyCode>>, mut mixer: ResMut<AudioPlusMixer>) {
    if input.just_pressed(KeyCode::F) {
        if mixer.get_volume(AudioPlusMixerChannel::Sfx) == 0. {
            mixer.set_volume(AudioPlusMixerChannel::Sfx, 1.);
        } else {
            mixer.set_volume(AudioPlusMixerChannel::Sfx, 0.);
        }
    }
    if input.just_pressed(KeyCode::M) {
        if mixer.get_volume(AudioPlusMixerChannel::Music) == 0. {
            mixer.set_volume(AudioPlusMixerChannel::Music, 1.);
        } else {
            mixer.set_volume(AudioPlusMixerChannel::Music, 0.);
        }
    }
}
