use asset_struct::prelude::*;
use bevy::prelude::*;
use jam::common::prelude::*;

#[derive(Component)]
pub struct Editable;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Character Controller".to_string(),
            width: 1280.,
            height: 720.,
            resizable: false,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins)
        .add_plugin(CommonPlugin)
        .add_plugin(quest1::Q1A1Plugin)
        .add_startup_system(init)
        .add_system(my_system)
        .run();
}

pub fn init(
    mut commands: Commands,
    mut asset_library: ResMut<AssetLibrary>,
    asset_server: Res<AssetServer>,
) {
    asset_library.load_assets(&asset_server);
    commands.spawn_bundle(Camera2dBundle::default());
}

pub fn my_system(
    input: Res<Input<KeyCode>>,
    mut ev_cutscene_start: EventWriter<CutsceneStartEvent<quest1::Q1A1Cutscene>>,
) {
    if input.just_pressed(KeyCode::Space) {
        ev_cutscene_start.send_default();
    }
}

pub mod quest1;
