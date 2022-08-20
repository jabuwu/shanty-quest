use bevy::prelude::*;
use jam::game::prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Player".to_string(),
            width: 1280.,
            height: 720.,
            resizable: false,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins)
        .add_plugin(jam::common::CommonPlugin)
        .add_plugin(jam::game::player::PlayerPlugin)
        .add_startup_system(init)
        .run();
}

pub fn init(mut commands: Commands, mut ev_player_spawn: EventWriter<PlayerSpawnEvent>) {
    commands.spawn_bundle(Camera2dBundle::default());
    ev_player_spawn.send_default();
}
