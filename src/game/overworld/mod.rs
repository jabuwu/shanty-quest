use crate::common::prelude::*;
use crate::game::prelude::*;
use crate::DEV_BUILD;
use audio_plus::prelude::*;
use bevy::prelude::*;

pub struct OverworldPlugin;

impl Plugin for OverworldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(player::PlayerPlugin)
            .add_plugin(world::WorldPlugin)
            .add_plugin(town::TownPlugin)
            .add_plugin(boat::BoatPlugin)
            .add_plugin(water_ring::WaterRingPlugin)
            .add_plugin(ocean::OceanPlugin)
            .add_plugin(healthbar::HealthbarPlugin)
            .add_plugin(character_controller::CharacterControllerPlugin)
            .add_plugin(attacks::AttacksPlugin)
            .add_plugin(damage::DamagePlugin)
            .add_plugin(cutscenes::CutscenesPlugin)
            .add_plugin(octopus::OctopusPlugin)
            .add_plugin(ui::OverworldUiPlugin)
            .add_plugin(camera::OverworldCameraPlugin)
            .add_plugin(entities::EntitiesPlugin)
            .add_plugin(trigger::TriggerPlugin)
            .add_plugin(enemy_spawns::EnemySpawnsPlugin)
            .add_plugin(threat_level::ThreatLevelPlugin)
            .add_plugin(turtle::TurtlePlugin)
            .add_plugin(experience::ExperiencePlugin)
            .add_plugin(damage_flash::DamageFlashPlugin)
            .add_plugin(damage_rum::DamageRumPlugin)
            .add_event::<OverworldEnterEvent>()
            .add_event::<WorldAmbienceSoundStopEvent>()
            .add_system_set(SystemSet::on_enter(AppState::Overworld).with_system(overworld_init))
            .add_system_set(SystemSet::on_update(AppState::Overworld).with_system(overworld_update))
            .add_system(overworld_init_after_ldtk)
            .add_system(overworld_sound_stop);
    }
}

#[derive(Default, Clone)]
pub struct OverworldEnterEvent;

#[derive(Default, Clone)]
pub struct WorldAmbienceSoundStopEvent;

#[derive(Component)]
pub struct WorldAmbienceSound;

fn overworld_init(
    mut screen_fade: ResMut<ScreenFade>,
    mut commands: Commands,
    mut ev_overworld_enter: EventWriter<OverworldEnterEvent>,
    mut ev_player_spawn: EventWriter<PlayerSpawnEvent>,
    mut ev_world_load: EventWriter<WorldLoadEvent>,
    mut overworld_camera: ResMut<OverworldCamera>,
    asset_library: Res<AssetLibrary>,
) {
    screen_fade.fade_in(1.);
    ev_overworld_enter.send_default();
    *overworld_camera = OverworldCamera::default();
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(Transform2::new().with_depth((DepthLayer::Camera, 0.)));
    ev_player_spawn.send_default();
    ev_world_load.send_default();
    commands
        .spawn()
        .insert(
            AudioPlusSource::new(asset_library.sound_effects.sfx_overworld_ambient.clone())
                .as_looping(),
        )
        .insert(WorldAmbienceSound);
    commands
        .spawn()
        .insert(
            AudioPlusSource::new(asset_library.sound_effects.sfx_overworld_music.clone())
                .as_looping(),
        )
        .insert(WorldAmbienceSound);
}

fn overworld_init_after_ldtk(
    mut ev_ui_spawn: EventWriter<OverworldUiSpawnEvent>,
    mut ev_spawn: EventReader<WorldLocationsSpawnEvent>,
) {
    for _ in ev_spawn.iter() {
        ev_ui_spawn.send_default();
    }
}

fn overworld_update(mut input: ResMut<Input<KeyCode>>, mut app_state: ResMut<State<AppState>>) {
    if DEV_BUILD {
        if input.just_pressed(KeyCode::Key0) {
            app_state.set(AppState::MainMenu).unwrap();
            input.reset(KeyCode::Key0);
        }
    }
}

fn overworld_sound_stop(
    mut ev_sound_stop: EventReader<WorldAmbienceSoundStopEvent>,
    mut query: Query<&mut AudioPlusSource, With<WorldAmbienceSound>>,
) {
    for _ in ev_sound_stop.iter() {
        for mut source in query.iter_mut() {
            source.stop();
        }
    }
}

pub mod attacks;
pub mod boat;
pub mod camera;
pub mod character_controller;
pub mod cutscenes;
pub mod damage;
pub mod damage_flash;
pub mod damage_rum;
pub mod enemy_spawns;
pub mod entities;
pub mod experience;
pub mod health;
pub mod healthbar;
pub mod ocean;
pub mod octopus;
pub mod player;
pub mod threat_level;
pub mod town;
pub mod trigger;
pub mod turtle;
pub mod ui;
pub mod water_ring;
pub mod world;
