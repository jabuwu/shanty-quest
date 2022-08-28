use asset_struct::prelude::*;
use bevy::prelude::*;
use jam::common::prelude::*;

#[derive(Component)]
pub struct Editable;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Timed Chance".to_string(),
            width: 1280.,
            height: 720.,
            resizable: false,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins)
        .add_plugin(CommonPlugin)
        .add_startup_system(init)
        .add_system(update)
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
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(Timed {
            chance: TimedChance::new(),
        });
}

pub fn update(mut query: Query<&mut Timed>, time: Res<Time>) {
    for mut timed in query.iter_mut() {
        if timed.chance.check(1.0, 1.0, time.delta_seconds()) {
            println!("ready!");
        }
    }
}
