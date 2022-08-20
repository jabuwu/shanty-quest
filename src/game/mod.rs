use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(player::PlayerPlugin)
            .add_system_set(SystemSet::on_enter(GameState::Game).with_system(game_init))
            .add_system_set(SystemSet::on_update(GameState::Game).with_system(game_update));
    }
}

pub fn game_init(
    mut commands: Commands,
    mut ev_player_spawn: EventWriter<PlayerSpawnEvent>,
) {
    commands.spawn_bundle(Camera2dBundle::default());
    ev_player_spawn.send_default();
}

pub fn game_update(mut input: ResMut<Input<KeyCode>>, mut game_state: ResMut<State<GameState>>) {
    if input.just_pressed(KeyCode::Escape) {
        game_state.set(GameState::MainMenu).unwrap();
        input.reset(KeyCode::Escape);
    }
}

pub mod player;
pub mod prelude;
