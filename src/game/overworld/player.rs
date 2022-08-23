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
            .add_system(player_enter_town)
            .add_system(player_camera.label(PlayerSystems::Camera))
            .add_system(player_set_attack);
    }
}

#[derive(Default, Clone, Copy)]
pub struct PlayerSpawnEvent;

#[derive(Component)]
pub struct Player {
    disabled: bool,
}

fn player_spawn(
    mut ev_spawn: EventReader<PlayerSpawnEvent>,
    mut ev_boat_spawn: EventWriter<BoatSpawnEvent>,
    mut commands: Commands,
    game_state: Res<GameState>,
) {
    for _ in ev_spawn.iter() {
        let entity = commands
            .spawn()
            .insert(Player { disabled: false })
            .insert(Label("Player".to_owned()))
            .insert(AudioPlusListener)
            .id();
        ev_boat_spawn.send(BoatSpawnEvent {
            entity: Some(entity),
            position: game_state.town.position + game_state.town.spawn_offset,
            special_attack: game_state.band_special_attack_type(),
            healthbar: false,
        });
    }
}

fn player_controls(
    mut query: Query<(&mut Boat, &GlobalTransform, &Player)>,
    mouse: Res<Mouse>,
    input: Res<Input<MouseButton>>,
    keys: Res<Input<KeyCode>>,
    cutscenes: Res<Cutscenes>,
) {
    if query.is_empty() {
        return;
    }
    for (mut boat, global_transform, player) in query.iter_mut() {
        if player.disabled || cutscenes.running() {
            boat.movement = Vec2::ZERO;
            continue;
        }
        let mut mouse_aim = (mouse.position - global_transform.translation().truncate()) / 200.;
        if mouse_aim.length_squared() == 0. {
            mouse_aim = Vec2::new(0.1, 0.);
        }
        boat.direction = Vec2::X.angle_between(mouse_aim);
        boat.movement = mouse_aim;
        if !input.pressed(MouseButton::Left) {
            boat.movement *= 0.05;
        }
        if keys.just_pressed(KeyCode::F) {
            boat.shoot = true;
        }
        if keys.just_pressed(KeyCode::D) {
            boat.special_shoot = true;
        }
    }
}

fn player_enter_town(
    mut game_state: ResMut<GameState>,
    island_query: Query<(Entity, &Town)>,
    mut player_query: Query<(Entity, &mut Player)>,
    transform_query: Query<&GlobalTransform>,
    mut ev_cutscene_enter_town: EventWriter<CutsceneStartEvent<EnterTownCutscene>>,
) {
    'outer: for (town_entity, island) in island_query.iter() {
        let town_position = if let Ok(town_transform) = transform_query.get(town_entity) {
            town_transform.translation().truncate()
        } else {
            continue;
        };
        for (player_entity, mut player) in player_query.iter_mut() {
            if player.disabled {
                continue;
            }
            let player_position = if let Ok(player_transform) = transform_query.get(player_entity) {
                player_transform.translation().truncate()
            } else {
                continue;
            };
            if player_position.distance(town_position) < 200. {
                player.disabled = true;
                ev_cutscene_enter_town.send(CutsceneStartEvent(EnterTownCutscene {
                    boat: Some(player_entity),
                    from: player_position,
                    to: town_position + Vec2::new(-10., -100.),
                }));
                game_state.town = island.town.clone();
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
    // TODO: remove debug
    for mut boat in query.iter_mut() {
        if input.just_pressed(KeyCode::Key1) {
            boat.special_attack = SpecialAttack::ShotgunCannons;
        }
        if input.just_pressed(KeyCode::Key2) {
            boat.special_attack = SpecialAttack::Shockwave;
        }
        if input.just_pressed(KeyCode::Key3) {
            boat.special_attack = SpecialAttack::DashAttack;
        }
    }
}
