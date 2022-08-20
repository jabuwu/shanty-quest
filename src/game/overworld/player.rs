use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerSpawnEvent>()
            .add_system(player_spawn)
            .add_system(player_move)
            .add_system(player_shoot)
            .add_system(player_enter_island);
    }
}

#[derive(Default, Clone, Copy)]
pub struct PlayerSpawnEvent;

#[derive(Component)]
pub struct Player;

fn player_spawn(
    mut ev_spawn: EventReader<PlayerSpawnEvent>,
    mut ev_boat_spawn: EventWriter<BoatSpawnEvent>,
    mut commands: Commands,
    game_state: Res<GameState>,
) {
    for _ in ev_spawn.iter() {
        let entity = commands
            .spawn()
            .insert(Player)
            .insert(Label("Player".to_owned()))
            .id();
        ev_boat_spawn.send(BoatSpawnEvent {
            entity: Some(entity),
            position: game_state.town.position + Vec2::new(-50., 0.),
        });
    }
}

fn player_move(
    mut query: Query<&mut Boat, With<Player>>,
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
    for mut boat in query.iter_mut() {
        boat.movement = movement;
    }
}

fn player_shoot(mut query: Query<&mut Boat, With<Player>>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::Space) {
        for mut boat in query.iter_mut() {
            boat.shoot = true;
        }
    }
}

fn player_enter_island(
    mut app_state: ResMut<State<AppState>>,
    mut game_state: ResMut<GameState>,
    island_query: Query<(Entity, &Island)>,
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
            if player_position.distance(island_position) < 20. {
                game_state.town = island.town.clone();
                app_state.set(AppState::GameTown).unwrap();
                break 'outer;
            }
        }
    }
}
