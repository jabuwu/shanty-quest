use crate::common::prelude::*;
use crate::game::prelude::*;
use audio_plus::prelude::*;
use bevy::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum PlayerSystems {
    Camera,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerSpawnEvent>()
            .add_system(player_spawn)
            .add_system(player_controls.before(BoatSystems::Update))
            .add_system(player_enter_island)
            .add_system(player_camera.label(PlayerSystems::Camera))
            .add_system(player_set_attack);
    }
}

#[derive(Default, Clone, Copy)]
pub struct PlayerSpawnEvent;

#[derive(Component)]
pub struct Player;

fn player_spawn(
    mut ev_spawn: EventReader<PlayerSpawnEvent>,
    mut ev_boat_spawn: EventWriter<BoatSpawnEvent>,
    mut ev_band_jam_spawn: EventWriter<BandJamSpawnEvent>,
    mut commands: Commands,
    game_state: Res<GameState>,
) {
    for _ in ev_spawn.iter() {
        let entity = commands
            .spawn()
            .insert(Player)
            .insert(Label("Player".to_owned()))
            .insert(AudioPlusListener)
            .insert(ShotgunCannons::default())
            .insert(Shockwave::default())
            .insert(DashAttack::default())
            .id();
        ev_boat_spawn.send(BoatSpawnEvent {
            entity: Some(entity),
            position: game_state.town.position + game_state.town.spawn_offset,
        });
        ev_band_jam_spawn.send(BandJamSpawnEvent {
            entity: Some(entity),
        });
    }
}

fn player_controls(
    mut query: Query<(&mut Boat, &mut BandJam), With<Player>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    if query.is_empty() {
        return;
    }
    let mut movement = Vec2::ZERO;
    if input.pressed(KeyCode::W) {
        movement.y += 1.;
    }
    if input.pressed(KeyCode::S) {
        movement.y -= 1.;
    }
    if input.pressed(KeyCode::A) {
        movement.x -= 1.;
    }
    if input.pressed(KeyCode::D) {
        movement.x += 1.;
    }
    if movement.length_squared() > 0. {
        movement = movement.normalize() * time.delta_seconds();
    }
    for (mut boat, mut band_jam) in query.iter_mut() {
        boat.movement = movement;
        band_jam.jamming = input.pressed(KeyCode::Space);
    }
}

fn player_enter_island(
    mut app_state: ResMut<State<AppState>>,
    mut game_state: ResMut<GameState>,
    island_query: Query<(Entity, &Town)>,
    player_query: Query<Entity, With<Player>>,
    transform_query: Query<&GlobalTransform>,
) {
    'outer: for (island_entity, island) in island_query.iter() {
        let island_position = if let Ok(island_transform) = transform_query.get(island_entity) {
            island_transform.translation().truncate()
        } else {
            continue;
        };
        for player_entity in player_query.iter() {
            let player_position = if let Ok(player_transform) = transform_query.get(player_entity) {
                player_transform.translation().truncate()
            } else {
                continue;
            };
            if player_position.distance(island_position) < 200. {
                game_state.town = island.town.clone();
                app_state.set(AppState::GameTown).unwrap();
                break 'outer;
            }
        }
    }
}

fn player_camera(
    player_query: Query<Entity, With<Player>>,
    camera_query: Query<Entity, With<Camera>>,
    mut transform_query: Query<&mut Transform2>,
) {
    let player_position = if let Ok(player_entity) = player_query.get_single() {
        if let Ok(player_transform) = transform_query.get(player_entity) {
            Some(player_transform.translation)
        } else {
            None
        }
    } else {
        None
    };
    if let Some(player_position) = player_position {
        for camera_entity in camera_query.iter() {
            if let Ok(mut camera_transform) = transform_query.get_mut(camera_entity) {
                camera_transform.translation = player_position;
            }
        }
    }
}

fn player_set_attack(mut query: Query<&mut Boat, With<Player>>, input: Res<Input<KeyCode>>) {
    for mut boat in query.iter_mut() {
        if input.just_pressed(KeyCode::Key1) {
            boat.attack = Attack::ShotgunCannons;
        }
        if input.just_pressed(KeyCode::Key2) {
            boat.attack = Attack::Shockwave;
        }
        if input.just_pressed(KeyCode::Key3) {
            boat.attack = Attack::DashAttack;
        }
    }
}
