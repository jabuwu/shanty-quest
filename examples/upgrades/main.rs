use asset_struct::prelude::*;
use bevy::prelude::*;
use jam::common::prelude::*;
use jam::game::prelude::*;
use jam::game::town::concert_hall::upgrades::*;

#[derive(Component)]
pub struct Editable;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Upgrades".to_string(),
                width: 1280.,
                height: 720.,
                resizable: false,
                ..default()
            },
            ..default()
        }))
        .add_plugin(CommonPlugin)
        .init_resource::<GameState>()
        .add_plugin(UpgradesPlugin)
        .add_startup_system(init)
        .run();
}

pub fn init(
    mut commands: Commands,
    mut asset_library: ResMut<AssetLibrary>,
    mut ev_upgrades_spawn: EventWriter<UpgradesSpawnEvent>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    asset_library.load_assets(&asset_server);
    asset_library.create_texture_atlases(texture_atlases.as_mut());
    asset_library.create_sound_effects();
    commands.spawn(Camera2dBundle::default());
    ev_upgrades_spawn.send_default();
}
