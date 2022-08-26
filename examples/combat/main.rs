use asset_struct::prelude::*;
use bevy::prelude::*;
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
        .insert_resource(WindowDescriptor {
            title: "Combat".to_string(),
            width: 1280.,
            height: 720.,
            resizable: false,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins)
        .add_plugin(CommonPlugin)
        .add_plugin(jam::game::overworld::player::PlayerPlugin)
        .add_plugin(jam::game::overworld::character_controller::CharacterControllerPlugin)
        .add_plugin(jam::game::overworld::boat::BoatPlugin)
        .add_plugin(jam::game::overworld::healthbar::HealthbarPlugin)
        .add_plugin(jam::game::overworld::damage::DamagePlugin)
        .add_plugin(jam::game::overworld::cutscenes::CutscenesPlugin)
        .add_plugin(jam::game::overworld::water_ring::WaterRingPlugin)
        .add_plugin(jam::game::overworld::camera::OverworldCameraPlugin)
        .add_plugin(jam::game::overworld::ocean::OceanPlugin)
        .add_plugin(jam::game::overworld::attacks::AttacksPlugin)
        .add_plugin(jam::game::overworld::octopus::OctopusPlugin)
        .add_plugin(jam::game::overworld::enemy_spawns::EnemySpawnsPlugin)
        .add_plugin(jam::game::overworld::threat_level::ThreatLevelPlugin)
        .add_plugin(jam::game::quests::QuestsPlugin)
        .init_resource::<jam::game::state::GameState>()
        .add_startup_system(init)
        .add_system(debug)
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
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(Transform2::new().with_depth((DepthLayer::Camera, 0.)));
    ev_player_spawn.send_default();
    ev_ocean_spawn.send_default();
}

fn debug(
    mut egui_context: ResMut<EguiContext>,
    mut ev_octopus_spawn: EventWriter<OctopusSpawnEvent>,
    mut ev_jagerossa_spawn: EventWriter<JagerossaSpawnEvent>,
    mut ev_ringo_spawn: EventWriter<RingoSpawnEvent>,
    mut ev_plank_spawn: EventWriter<PlankSpawnEvent>,
    mut ev_davy_spawn: EventWriter<DavySpawnEvent>,
    player_query: Query<&GlobalTransform, With<Player>>,
    input: Res<Input<KeyCode>>,
    mut world_locations: ResMut<WorldLocations>,
    mut game_state: ResMut<GameState>,
) {
    let player_position = if let Ok(player_transform) = player_query.get_single() {
        player_transform.translation().truncate()
    } else {
        Vec2::ZERO
    };
    egui::Window::new("Combat").show(egui_context.ctx_mut(), |ui| {
        ui.label("1) Octopus");
        ui.label("6) Jagerossa");
        ui.label("7) Ringo");
        ui.label("8) Plank");
        ui.label("9) Davy");
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
                    ui.label(format!("{}", $value));
                    if ui.button("+").clicked() {
                        $value += 1;
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
    if input.just_pressed(KeyCode::Key1) {
        let spawn_pos = Vec2::from_angle(rand::random::<f32>() * std::f32::consts::TAU) * 500.;
        ev_octopus_spawn.send(OctopusSpawnEvent {
            entity: None,
            position: player_position + spawn_pos,
        });
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
}
