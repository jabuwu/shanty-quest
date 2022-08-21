use asset_struct::prelude::*;
use audio_plus::prelude::*;
use bevy::prelude::*;
use jam::common::prelude::*;
use jam::game::prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Band Jam".to_string(),
            width: 1280.,
            height: 720.,
            resizable: false,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins)
        .add_plugin(jam::common::CommonPlugin)
        .add_plugin(jam::game::overworld::character_controller::CharacterControllerPlugin)
        .add_plugin(jam::game::overworld::band_jam::BandJamPlugin)
        .add_startup_system(init)
        .add_system(loading_update)
        .add_system(player_move)
        .run();
}

#[derive(Component)]
pub struct Player;

fn init(
    mut commands: Commands,
    mut asset_library: ResMut<AssetLibrary>,
    asset_server: Res<AssetServer>,
) {
    asset_library.load_assets(&asset_server);
    commands.spawn_bundle(Camera2dBundle::default());
}

#[derive(Default)]
struct LoadingUpdateState {
    loaded: bool,
}

fn loading_update(
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
    asset_server: Res<AssetServer>,
    mut state: Local<LoadingUpdateState>,
    mut ev_band_jam_spawn: EventWriter<BandJamSpawnEvent>,
) {
    use bevy::asset::LoadState;
    if state.loaded {
        return;
    }
    match asset_library.load_state(&asset_server) {
        LoadState::Failed => {
            panic!("Failed to load assets.");
        }
        LoadState::Loaded => {
            state.loaded = true;
            let entity = commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Vec2::new(32., 32.).into(),
                        color: Color::GREEN,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Transform2::from_xy(0., 0.))
                .insert(Collision {
                    shape: CollisionShape::Rect {
                        size: Vec2::new(32., 32.),
                    },
                    flags: 1,
                })
                .insert(Player)
                .insert(CharacterController {
                    movement: Vec2::ZERO,
                    speed: 300.,
                })
                .insert(AudioPlusListener)
                .id();
            ev_band_jam_spawn.send(BandJamSpawnEvent {
                entity: Some(entity),
            });
        }
        _ => {}
    }
}

fn player_move(
    mut query: Query<(&mut CharacterController, &mut BandJam), With<Player>>,
    input: Res<Input<KeyCode>>,
) {
    for (mut character_controller, mut band_jam) in query.iter_mut() {
        character_controller.movement = Vec2::ZERO;
        if input.pressed(KeyCode::W) {
            character_controller.movement.y += 1.;
        }
        if input.pressed(KeyCode::S) {
            character_controller.movement.y -= 1.;
        }
        if input.pressed(KeyCode::A) {
            character_controller.movement.x -= 1.;
        }
        if input.pressed(KeyCode::D) {
            character_controller.movement.x += 1.;
        }
        band_jam.jamming = input.pressed(KeyCode::Space);
    }
}
