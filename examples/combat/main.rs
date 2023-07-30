use asset_struct::prelude::*;
use bevy::{prelude::*, window::WindowResolution};
use bevy_egui::{egui, EguiContext};
use jam::{
    common::prelude::*,
    game::{
        prelude::*,
        quests::{
            davy::davy::DavySpawnEvent, jagerossa::jagerossa::JagerossaSpawnEvent,
            plank::plank::PlankSpawnEvent, ringo::ringo::RingoSpawnEvent,
        },
    },
};

#[derive(Component)]
pub struct Editable;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Combat".to_string(),
                resolution: WindowResolution::new(1280., 720.),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            CommonPlugin,
            jam::game::overworld::player::PlayerPlugin,
            jam::game::overworld::character_controller::CharacterControllerPlugin,
            jam::game::overworld::boat::BoatPlugin,
            jam::game::overworld::healthbar::HealthbarPlugin,
            jam::game::overworld::damage::DamagePlugin,
            jam::game::overworld::cutscenes::CutscenesPlugin,
            jam::game::overworld::water_ring::WaterRingPlugin,
            jam::game::overworld::camera::OverworldCameraPlugin,
            jam::game::overworld::ocean::OceanPlugin,
            jam::game::overworld::attacks::AttacksPlugin,
            jam::game::overworld::octopus::OctopusPlugin,
            jam::game::overworld::turtle::TurtlePlugin,
            jam::game::overworld::enemy_spawns::EnemySpawnsPlugin,
            jam::game::overworld::threat_level::ThreatLevelPlugin,
        ))
        .add_plugins((
            jam::game::overworld::experience::ExperiencePlugin,
            jam::game::overworld::ui::OverworldUiPlugin,
            jam::game::overworld::damage_flash::DamageFlashPlugin,
            jam::game::overworld::damage_rum::DamageRumPlugin,
            jam::game::quests::QuestsPlugin,
            jam::game::town::outside::rum_refill::RumRefillPlugin,
        ))
        .init_resource::<jam::game::state::GameState>()
        .add_systems(Startup, init)
        .add_systems(Update, debug)
        .run();
}

pub fn init(
    mut commands: Commands,
    mut asset_library: ResMut<AssetLibrary>,
    asset_server: Res<AssetServer>,
    mut ev_player_spawn: EventWriter<PlayerSpawnEvent>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut ev_ocean_spawn: EventWriter<OceanSpawnEvent>,
) {
    asset_library.load_assets(&asset_server);
    asset_library.create_texture_atlases(texture_atlases.as_mut());
    asset_library.create_sound_effects();
    commands.spawn((
        Camera2dBundle::default(),
        Transform2::new().with_depth((DepthLayer::Camera, 0.)),
    ));
    ev_player_spawn.send_default();
    ev_ocean_spawn.send_default();
}

#[derive(Default)]
struct DebugState {
    stress_test: bool,
    stress_pressed: bool,
}

fn debug(
    mut egui_query: Query<&mut EguiContext>,
    mut ev_octopus_spawn: EventWriter<OctopusSpawnEvent>,
    mut ev_turtle_spawn: EventWriter<TurtleSpawnEvent>,
    mut ev_jagerossa_spawn: EventWriter<JagerossaSpawnEvent>,
    mut ev_ringo_spawn: EventWriter<RingoSpawnEvent>,
    mut ev_plank_spawn: EventWriter<PlankSpawnEvent>,
    mut ev_davy_spawn: EventWriter<DavySpawnEvent>,
    player_query: Query<&GlobalTransform, With<Player>>,
    mut player_health_query: Query<&mut Health, With<Player>>,
    mut input: ResMut<Input<KeyCode>>,
    mut world_locations: ResMut<WorldLocations>,
    mut game_state: ResMut<GameState>,
    mut overworld_camera: ResMut<OverworldCamera>,
    mut local: Local<DebugState>,
    mut commands: Commands,
) {
    let player_position = if let Ok(player_transform) = player_query.get_single() {
        player_transform.translation().truncate()
    } else {
        Vec2::ZERO
    };
    for mut player_health in player_health_query.iter_mut() {
        player_health.value = player_health.max;
    }
    let Some(mut egui_context) = egui_query.get_single_mut().ok() else { return };
    egui::Window::new("Combat").show(egui_context.get_mut(), |ui| {
        ui.label("1) Octopus");
        ui.label("2) Turtle");
        ui.label("6) Jagerossa");
        ui.label("7) Ringo");
        ui.label("8) Plank");
        ui.label("9) Davy");
        ui.label("");
        ui.label(if overworld_camera.is_arena_enabled() {
            "L) Unlock Cam"
        } else {
            "L) Lock Cam"
        });
        ui.label("");

        macro_rules! attack_setting {
            ($str:expr, $value:expr) => {
                ui.label($str);
                ui.horizontal(|ui| {
                    if ui.button("-").clicked() {
                        if $value > 0 {
                            $value -= 1;
                        }
                    }
                    if $value == 6 {
                        ui.label("boss");
                    } else {
                        ui.label(format!("{}", $value));
                    }
                    if ui.button("+").clicked() {
                        if $value < 6 {
                            $value += 1;
                        }
                    }
                });
            };
        }
        attack_setting!("Forward Cannons", game_state.attacks.forward_cannons);
        attack_setting!("Shotgun Cannons", game_state.attacks.shotgun_cannons);
        attack_setting!("Shockwave", game_state.attacks.shockwave);
        attack_setting!("Bombs", game_state.attacks.bombs);
        attack_setting!("Kraken", game_state.attacks.kraken);
    });
    if local.stress_test {
        if local.stress_pressed {
            input.release(KeyCode::Key1);
            input.release(KeyCode::Key6);
            input.release(KeyCode::Key7);
            input.release(KeyCode::Key8);
            input.release(KeyCode::Key9);
        } else {
            input.press(KeyCode::Key1);
            input.press(KeyCode::Key6);
            input.press(KeyCode::Key7);
            input.press(KeyCode::Key8);
            input.press(KeyCode::Key9);
        }
        local.stress_pressed = !local.stress_pressed;
        for _ in 0..100 {
            commands.spawn((
                Transform2Bundle {
                    ..Default::default()
                },
                Hurtbox {
                    shape: CollisionShape::Rect {
                        size: Vec2::new(9999999., 9999999.),
                    },
                    for_entity: None,
                    auto_despawn: true,
                    //flags: std::u32::MAX,
                    flags: DAMAGE_FLAG_ENEMY,
                    //flags: DAMAGE_FLAG_PLAYER,
                    knockback_type: HurtboxKnockbackType::None,
                    damage: 999.,
                },
            ));
        }
    }
    if input.just_pressed(KeyCode::Key1) {
        let level = if input.pressed(KeyCode::ShiftLeft) {
            OctopusLevel::Hard
        } else if input.pressed(KeyCode::ControlLeft) {
            OctopusLevel::Medium
        } else {
            OctopusLevel::Easy
        };
        let spawn_pos = Vec2::from_angle(rand::random::<f32>() * std::f32::consts::TAU) * 500.;
        ev_octopus_spawn.send(OctopusSpawnEvent {
            entity: None,
            position: player_position + spawn_pos,
            level,
        });
    }
    if input.just_pressed(KeyCode::Key2) {
        let level = if input.pressed(KeyCode::ShiftLeft) {
            TurtleLevel::Hard
        } else if input.pressed(KeyCode::ControlLeft) {
            TurtleLevel::Medium
        } else {
            TurtleLevel::Easy
        };
        let spawn_pos = Vec2::from_angle(rand::random::<f32>() * std::f32::consts::TAU) * 500.;
        ev_turtle_spawn.send(TurtleSpawnEvent {
            entity: None,
            position: player_position + spawn_pos,
            level,
        });
    }
    if input.just_pressed(KeyCode::L) {
        if overworld_camera.is_arena_enabled() {
            overworld_camera.arena_disable();
        } else {
            if input.pressed(KeyCode::ShiftLeft) {
                overworld_camera.arena_enable(player_position, Vec2::new(1280. * 1.3, 768. * 1.3));
            } else if input.pressed(KeyCode::ControlLeft) {
                overworld_camera.arena_enable(player_position, Vec2::new(1280. * 1.1, 768. * 1.1));
            } else {
                overworld_camera.arena_enable(player_position, Vec2::new(1280., 768.));
            }
        }
    }
    if input.just_pressed(KeyCode::Key6) {
        world_locations.clear();
        world_locations.add(
            "JagerossaSpawn",
            player_position + Vec2::new(500., 0.),
            Vec2::ZERO,
        );
        world_locations.add(
            "JagerossaMoveTo",
            player_position + Vec2::new(400., 0.),
            Vec2::ZERO,
        );
        ev_jagerossa_spawn.send(JagerossaSpawnEvent);
    }
    if input.just_pressed(KeyCode::Key7) {
        world_locations.clear();
        world_locations.add(
            "RingoSpawn",
            player_position + Vec2::new(500., 0.),
            Vec2::ZERO,
        );
        world_locations.add(
            "RingoMoveTo",
            player_position + Vec2::new(400., 0.),
            Vec2::ZERO,
        );
        ev_ringo_spawn.send(RingoSpawnEvent);
    }
    if input.just_pressed(KeyCode::Key8) {
        world_locations.clear();
        world_locations.add(
            "PlankSpawn",
            player_position + Vec2::new(500., 0.),
            Vec2::ZERO,
        );
        world_locations.add(
            "PlankMoveTo",
            player_position + Vec2::new(400., 0.),
            Vec2::ZERO,
        );
        ev_plank_spawn.send(PlankSpawnEvent);
    }
    if input.just_pressed(KeyCode::Key9) {
        world_locations.clear();
        world_locations.add(
            "DavySpawn",
            player_position + Vec2::new(500., 0.),
            Vec2::ZERO,
        );
        world_locations.add(
            "DavyMoveTo",
            player_position + Vec2::new(400., 0.),
            Vec2::ZERO,
        );
        ev_davy_spawn.send(DavySpawnEvent);
    }
    if input.just_pressed(KeyCode::F5) {
        local.stress_test = !local.stress_test;
    }
}
