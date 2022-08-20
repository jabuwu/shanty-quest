use app_state::prelude::*;

#[derive(AppState, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    #[default]
    Loading,
    MainMenu,
    Game,
}
