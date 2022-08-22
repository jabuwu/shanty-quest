use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;

pub struct OverworldPlugin;

impl Plugin for OverworldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(player::PlayerPlugin)
            .add_plugin(world::WorldPlugin)
            .add_plugin(town::TownPlugin)
            .add_plugin(boat::BoatPlugin)
            .add_plugin(enemy::EnemyPlugin)
            .add_plugin(water_ring::WaterRingPlugin)
            .add_plugin(ocean::OceanPlugin)
            .add_plugin(healthbar::HealthbarPlugin)
            .add_plugin(character_controller::CharacterControllerPlugin)
            .add_plugin(band_jam::BandJamPlugin)
            .add_plugin(attacks::AttacksPlugin)
            .add_system_set(SystemSet::on_enter(AppState::Overworld).with_system(overworld_init))
            .add_system_set(
                SystemSet::on_update(AppState::Overworld).with_system(overworld_update),
            );
    }
}

pub fn overworld_init(
    mut commands: Commands,
    mut ev_player_spawn: EventWriter<PlayerSpawnEvent>,
    mut ev_enemy_spawn: EventWriter<EnemySpawnEvent>,
    mut ev_world_load: EventWriter<WorldLoadEvent>,
) {
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(Transform2::new().with_depth((DepthLayer::Camera, 0.)));
    ev_player_spawn.send_default();
    ev_enemy_spawn.send_default();
    ev_world_load.send_default();
}

pub fn overworld_update(mut input: ResMut<Input<KeyCode>>, mut app_state: ResMut<State<AppState>>) {
    if input.just_pressed(KeyCode::Escape) {
        app_state.set(AppState::MainMenu).unwrap();
        input.reset(KeyCode::Escape);
    }
}

pub mod attacks;
pub mod band_jam;
pub mod boat;
pub mod character_controller;
pub mod depth_layers;
pub mod enemy;
pub mod health;
pub mod healthbar;
pub mod ocean;
pub mod player;
pub mod town;
pub mod water_ring;
pub mod world;
