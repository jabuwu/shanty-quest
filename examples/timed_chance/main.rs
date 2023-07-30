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
                title: "Timed Chance".to_string(),
                resolution: WindowResolution::new(1280., 720.),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(CommonPlugin)
        .add_systems(Startup, init)
        .add_systems(Update, update)
        .run();
}

#[derive(Component)]
pub struct Timed {
    chance: TimedChance,
}

pub fn init(
    mut commands: Commands,
    mut asset_library: ResMut<AssetLibrary>,
    asset_server: Res<AssetServer>,
) {
    asset_library.load_assets(&asset_server);
    commands.spawn((
        Camera2dBundle::default(),
        Timed {
            chance: TimedChance::new(),
        },
    ));
}

pub fn update(mut query: Query<&mut Timed>, time: Res<Time>) {
    for mut timed in query.iter_mut() {
        if timed.chance.check(1.0, 1.0, time.delta_seconds()) {
            println!("ready!");
        }
    }
}
