use global_state::prelude::*;

#[derive(GlobalState, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    #[default]
    Loading,
    MainMenu,
    GameOverworld,
    GameTown,
}
