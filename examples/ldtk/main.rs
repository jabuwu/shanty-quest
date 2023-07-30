use asset_struct::prelude::*;
use bevy::{prelude::*, window::WindowResolution};
use jam::common::prelude::*;

#[derive(Component)]
pub struct Editable;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "LDTK".to_string(),
                resolution: WindowResolution::new(1280., 720.),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(jam::common::CommonPlugin)
        .add_systems(Startup, init)
        .run();
}

pub fn init(
    mut commands: Commands,
    mut asset_library: ResMut<AssetLibrary>,
    asset_server: Res<AssetServer>,
    mut ev_spawn_ldtk: EventWriter<LdtkSpawnEvent>,
) {
    asset_library.load_assets(&asset_server);
    commands.spawn(Camera2dBundle::default());
    ev_spawn_ldtk.send(LdtkSpawnEvent {
        entity: None,
        asset: asset_library.level.clone(),
        position: Vec2::new(-800., 350.),
    });
}
