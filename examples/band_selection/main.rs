use asset_struct::prelude::*;
use bevy::prelude::*;
use jam::common::prelude::*;
use jam::game::prelude::*;
use jam::game::town::concert_hall::band_selection::*;

#[derive(Component)]
pub struct Editable;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Band Selection".to_string(),
            width: 1280.,
            height: 720.,
            resizable: false,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins)
        .add_plugin(CommonPlugin)
        .init_resource::<GameState>()
        .add_plugin(BandSelectionPlugin)
        .add_startup_system(init)
        .run();
}

pub fn init(
    mut commands: Commands,
    mut asset_library: ResMut<AssetLibrary>,
    mut ev_band_selection_spawn: EventWriter<BandSelectionSpawnEvent>,
    asset_server: Res<AssetServer>,
) {
    asset_library.load_assets(&asset_server);
    commands.spawn_bundle(Camera2dBundle::default());
    ev_band_selection_spawn.send_default();
}
