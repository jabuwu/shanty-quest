use crate::common::prelude::*;
use crate::game::prelude::*;
use audio_plus::prelude::*;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerSpawnEvent>()
            .add_system(player_spawn)
            .add_system(player_controls.before(BoatSystems::Update))
            .add_system(player_enter_town)
            .add_system(player_upgrade_attack)
            .add_system(player_invincibility)
            .add_system(player_damage);
    }
}

#[derive(Default, Clone, Copy)]
pub struct PlayerSpawnEvent;

#[derive(Component)]
pub struct Player {
    disabled: bool,
    invincibility: f32,
    dead: bool,
}

fn player_spawn(
    mut ev_spawn: EventReader<PlayerSpawnEvent>,
    mut ev_boat_spawn: EventWriter<BoatSpawnEvent>,
    mut commands: Commands,
    game_state: Res<GameState>,
    mut ev_cutscene_exit_town: EventWriter<CutsceneStartEvent<ExitTownCutscene>>,
) {
    for _ in ev_spawn.iter() {
        let entity = commands
            .spawn()
            .insert(Player {
                disabled: false,
                invincibility: 0.,
                dead: false,
            })
            .insert(Label("Player".to_owned()))
            .insert(AudioPlusListener)
            .id();
        ev_boat_spawn.send(BoatSpawnEvent {
            entity: Some(entity),
            position: game_state.town.position + game_state.town.spawn_offset,
            attack: Attacks {
                forward_cannons: 1,
                ..Default::default()
            },
            healthbar: false,
            player: true,
            health: 20.,
            speed: 250.,
            attack_cooldown: 0.48,
            knockback_resistance: 0.2,
        });
        if !game_state.quests.block_town_exit_cutscene() {
            ev_cutscene_exit_town.send(CutsceneStartEvent(ExitTownCutscene {
                boat: Some(entity),
                to: game_state.town.position + game_state.town.spawn_offset,
                from: game_state.town.position + Vec2::new(-10., -100.),
            }));
        }
    }
}

fn player_controls(
    mut query: Query<(&mut Boat, &GlobalTransform, &Player)>,
    mouse: Res<Mouse>,
    input: Res<Input<MouseButton>>,
    keys: Res<Input<KeyCode>>,
    cutscenes: Res<Cutscenes>,
    game_state: Res<GameState>,
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
            boat.movement *= 0.005;
        }
        boat.dash = keys.pressed(KeyCode::Space);
        if keys.just_pressed(KeyCode::F) {
            boat.shoot = !boat.shoot;
        }
        boat.attacks = game_state.attacks;
    }
}

fn player_enter_town(
    mut game_state: ResMut<GameState>,
    town_query: Query<(Entity, &Town)>,
    mut player_query: Query<(Entity, &mut Player)>,
    transform_query: Query<&GlobalTransform>,
    mut ev_cutscene_enter_town: EventWriter<CutsceneStartEvent<EnterTownCutscene>>,
    cutscenes: Res<Cutscenes>,
    state_time: Res<StateTime<AppState>>,
) {
    if cutscenes.running() || state_time.just_entered() || game_state.quests.block_town_enter() {
        return;
    }
    'outer: for (town_entity, town) in town_query.iter() {
        let town_position = if let Ok(town_transform) = transform_query.get(town_entity) {
            town_transform.translation().truncate()
        } else {
            continue;
        };
        for (player_entity, mut player) in player_query.iter_mut() {
            if player.disabled {
                continue;
            }
            if town.block_timer > 0. {
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
                game_state.town = town.town.clone();
                break 'outer;
            }
        }
    }
}

fn player_upgrade_attack(input: Res<Input<KeyCode>>, mut game_state: ResMut<GameState>) {
    // TODO: remove debug
    if input.just_pressed(KeyCode::F1) {
        game_state.attacks = Attacks {
            forward_cannons: 1,
            shotgun_cannons: 1,
            shockwave: 1,
            bombs: 1,
            kraken: 1,
        };
    }
}

fn player_invincibility(mut crate_query: Query<(&mut Player, &mut Boat)>, time: Res<Time>) {
    for (mut player, mut boat) in crate_query.iter_mut() {
        player.invincibility -= time.delta_seconds();
        player.invincibility = player.invincibility.max(0.);
        if player.invincibility <= 0. {
            boat.opacity = 1.;
        } else {
            boat.opacity = 0.5;
        }
    }
}

fn player_damage(
    mut ev_damage: EventReader<DamageEvent>,
    mut crate_query: Query<(&mut Health, &mut Player)>,
    mut ev_death_cutscene: EventWriter<CutsceneStartEvent<DeathCutscene>>,
    cutscenes: Res<Cutscenes>,
) {
    for event in ev_damage.iter() {
        if let Ok((mut health, mut player)) = crate_query.get_mut(event.hit) {
            if player.invincibility <= 0. {
                if !cutscenes.running() {
                    health.damage(1.);
                }
                if !player.dead && health.dead() {
                    player.dead = true;
                    ev_death_cutscene.send_default();
                }
                player.invincibility = 0.3;
            }
        }
    }
}
