use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;

pub struct OverworldPlugin;

impl Plugin for OverworldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(player::PlayerPlugin)
            .add_plugin(world::WorldPlugin)
            .add_plugin(island::IslandPlugin)
            .add_plugin(boat::BoatPlugin)
            .add_plugin(enemy::EnemyPlugin)
            .add_plugin(cannon_ball::CannonBallPlugin)
            .add_system_set(
                SystemSet::on_enter(AppState::GameOverworld).with_system(overworld_init),
            )
            .add_system_set(
                SystemSet::on_update(AppState::GameOverworld).with_system(overworld_update),
            );
    }
}

pub fn overworld_init(
    mut commands: Commands,
    mut ev_player_spawn: EventWriter<PlayerSpawnEvent>,
    mut ev_enemy_spawn: EventWriter<EnemySpawnEvent>,
    mut ev_world_load: EventWriter<WorldLoadEvent>,
    mut ev_spawn_ldtk: EventWriter<LdtkSpawnEvent>,
    asset_library: Res<AssetLibrary>,
) {
    commands.spawn_bundle(Camera2dBundle::default());
    ev_player_spawn.send_default();
    ev_enemy_spawn.send_default();
    ev_world_load.send_default();
    ev_spawn_ldtk.send(LdtkSpawnEvent {
        entity: None,
        asset: asset_library.level_test.clone(),
        position: Vec2::new(-800., 350.),
    });
}

pub fn overworld_update(mut input: ResMut<Input<KeyCode>>, mut app_state: ResMut<State<AppState>>) {
    if input.just_pressed(KeyCode::Escape) {
        app_state.set(AppState::MainMenu).unwrap();
        input.reset(KeyCode::Escape);
    }
}

pub mod boat;
pub mod cannon_ball;
pub mod enemy;
pub mod island;
pub mod player;
pub mod world;
