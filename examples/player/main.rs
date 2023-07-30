use bevy::{prelude::*, window::WindowResolution};
use jam::game::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Player".to_string(),
                resolution: WindowResolution::new(1280., 720.),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .init_resource::<GameState>()
        .add_plugins((
            jam::common::CommonPlugin,
            jam::game::overworld::player::PlayerPlugin,
            jam::game::overworld::damage::DamagePlugin,
            jam::game::overworld::healthbar::HealthbarPlugin,
            jam::game::overworld::cutscenes::CutscenesPlugin,
            jam::game::overworld::boat::BoatPlugin,
            jam::game::overworld::water_ring::WaterRingPlugin,
        ))
        .add_systems(Startup, init)
        .run();
}

pub fn init(mut commands: Commands, mut ev_player_spawn: EventWriter<PlayerSpawnEvent>) {
    commands.spawn(Camera2dBundle::default());
    ev_player_spawn.send_default();
}
