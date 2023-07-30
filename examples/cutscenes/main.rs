use asset_struct::prelude::*;
use bevy::{prelude::*, window::WindowResolution};
use jam::common::prelude::*;
use quest1::Q1A1Cutscene;

#[derive(Component)]
pub struct Editable;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Cutscenes".to_string(),
                resolution: WindowResolution::new(1280., 720.),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins((CommonPlugin, quest1::Q1A1Plugin))
        .add_systems(Startup, init)
        .add_systems(Update, my_system)
        .run();
}

pub fn init(
    mut commands: Commands,
    mut asset_library: ResMut<AssetLibrary>,
    asset_server: Res<AssetServer>,
) {
    asset_library.load_assets(&asset_server);
    commands.spawn(Camera2dBundle::default());
}

pub fn my_system(
    input: Res<Input<KeyCode>>,
    mut ev_cutscene_start: EventWriter<CutsceneStartEvent<quest1::Q1A1Cutscene>>,
) {
    if input.just_pressed(KeyCode::Space) {
        let birdup = rand::random::<f32>();
        println!("birdup: {}", birdup);
        ev_cutscene_start.send(CutsceneStartEvent(Q1A1Cutscene { birdup, time: 0. }));
    }
}

pub mod quest1;
