use asset_struct::prelude::*;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use jam::common::prelude::*;
use jam::game::prelude::*;
use jam::game::town::concert_hall::band_selection::*;

#[derive(Component)]
pub struct Editable;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Band Selection".to_string(),
                resolution: WindowResolution::new(1280., 720.),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins((CommonPlugin, BandSelectionPlugin))
        .init_resource::<GameState>()
        .add_systems(Startup, init)
        .run();
}

pub fn init(
    mut commands: Commands,
    mut asset_library: ResMut<AssetLibrary>,
    mut ev_band_selection_spawn: EventWriter<BandSelectionSpawnEvent>,
    asset_server: Res<AssetServer>,
) {
    asset_library.load_assets(&asset_server);
    commands.spawn(Camera2dBundle::default());
    ev_band_selection_spawn.send_default();
}
