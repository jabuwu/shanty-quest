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
            .add_plugin(water_ring::WaterRingPlugin)
            .add_plugin(ocean::OceanPlugin)
            .add_plugin(character_controller::CharacterControllerPlugin)
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
    mut ev_ldtk_spawn: EventWriter<LdtkSpawnEvent>,
    mut ev_ocean_spawn: EventWriter<OceanSpawnEvent>,
    asset_library: Res<AssetLibrary>,
) {
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(Transform2::new().with_depth((DepthLayer::Camera, 0.)));
    ev_player_spawn.send_default();
    ev_enemy_spawn.send_default();
    ev_world_load.send_default();
    ev_ocean_spawn.send_default();
    ev_ldtk_spawn.send(LdtkSpawnEvent {
        entity: None,
        asset: asset_library.level_test.clone(),
        position: Vec2::new(-800., 350.),
    });
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Vec2::new(128., 128.).into(),
                color: Color::RED,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Transform2::from_xy(0., -200.).with_depth((DepthLayer::Entity, 0.)))
        .insert(Collision {
            shape: CollisionShape::Rect {
                size: Vec2::new(128., 128.),
            },
            flags: 1,
        })
        .insert(YDepth);
}

pub fn overworld_update(mut input: ResMut<Input<KeyCode>>, mut app_state: ResMut<State<AppState>>) {
    if input.just_pressed(KeyCode::Escape) {
        app_state.set(AppState::MainMenu).unwrap();
        input.reset(KeyCode::Escape);
    }
}

pub mod boat;
pub mod cannon_ball;
pub mod character_controller;
pub mod enemy;
pub mod island;
pub mod ocean;
pub mod player;
pub mod water_ring;
pub mod world;
