use bevy::prelude::*;
use global_state::prelude::*;

#[derive(GlobalState, Default, Debug, Clone, Eq, PartialEq, Hash, Resource, States)]
pub enum AppState {
    #[default]
    Loading,
    MainMenu,
    IntroCutscene,
    OutroCutscene,
    Overworld,
    TownOutside,
    TownTavern,
    TownMayor,
    TownConcertHall,
    Dead,
}

impl AppState {
    pub fn is_town(&self) -> bool {
        match *self {
            Self::Loading => false,
            Self::MainMenu => false,
            Self::IntroCutscene => false,
            Self::OutroCutscene => false,
            Self::Overworld => false,
            Self::TownOutside => true,
            Self::TownTavern => true,
            Self::TownMayor => true,
            Self::TownConcertHall => true,
            Self::Dead => false,
        }
    }
}
