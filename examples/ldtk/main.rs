use asset_struct::prelude::*;
use bevy::prelude::*;
use jam::common::prelude::*;

#[derive(Component)]
pub struct Editable;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "LDTK".to_string(),
            width: 1280.,
            height: 720.,
            resizable: false,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins)
        .add_plugin(jam::common::CommonPlugin)
        .add_startup_system(init)
        .run();
}

pub fn init(
    mut commands: Commands,
    mut asset_library: ResMut<AssetLibrary>,
    asset_server: Res<AssetServer>,
    mut ev_spawn_ldtk: EventWriter<LdtkSpawnEvent>,
) {
    asset_library.load_assets(&asset_server);
    commands.spawn_bundle(Camera2dBundle::default());
    ev_spawn_ldtk.send(LdtkSpawnEvent {
        entity: None,
        asset: asset_library.level.clone(),
        position: Vec2::new(-800., 350.),
    });
}
