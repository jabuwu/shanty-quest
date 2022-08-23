use crate::common::prelude::*;
use crate::game::prelude::*;
use audio_plus::prelude::*;
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
            .add_plugin(attacks::AttacksPlugin)
            .add_plugin(damage::DamagePlugin)
            .add_plugin(cutscenes::CutscenesPlugin)
            .add_plugin(octopus::OctopusPlugin)
            .add_event::<WorldAmbienceSoundStopEvent>()
            .add_system_set(SystemSet::on_enter(AppState::Overworld).with_system(overworld_init))
            .add_system_set(SystemSet::on_update(AppState::Overworld).with_system(overworld_update))
            .add_system(overworld_sound_stop);
    }
}

#[derive(Default, Clone)]
pub struct WorldAmbienceSoundStopEvent;

#[derive(Component)]
pub struct WorldAmbienceSound;

fn overworld_init(
    mut screen_fade: ResMut<ScreenFade>,
    mut commands: Commands,
    mut ev_player_spawn: EventWriter<PlayerSpawnEvent>,
    mut ev_enemy_spawn: EventWriter<EnemySpawnEvent>,
    mut ev_octopus_spawn: EventWriter<OctopusSpawnEvent>,
    mut ev_world_load: EventWriter<WorldLoadEvent>,
    asset_library: Res<AssetLibrary>,
    mut ev_cutscene_example_dialogue: EventWriter<CutsceneStartEvent<ExampleDialogueCutscene>>,
    mut game_state: ResMut<GameState>,
) {
    screen_fade.fade_in(1.);
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(Transform2::new().with_depth((DepthLayer::Camera, 0.)));
    ev_player_spawn.send_default();
    ev_enemy_spawn.send(EnemySpawnEvent {
        position: Vec2::new(500., -400.),
    });
    ev_octopus_spawn.send(OctopusSpawnEvent {
        position: Vec2::new(100., -400.),
    });
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
    if !game_state.showed_example_text {
        ev_cutscene_example_dialogue.send_default();
        game_state.showed_example_text = true;
    }
}

fn overworld_update(mut input: ResMut<Input<KeyCode>>, mut app_state: ResMut<State<AppState>>) {
    if input.just_pressed(KeyCode::Escape) {
        app_state.set(AppState::MainMenu).unwrap();
        input.reset(KeyCode::Escape);
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
pub mod character_controller;
pub mod cutscenes;
pub mod damage;
pub mod depth_layers;
pub mod enemy;
pub mod health;
pub mod healthbar;
pub mod ocean;
pub mod octopus;
pub mod player;
pub mod town;
pub mod water_ring;
pub mod world;
