use global_state::prelude::*;

#[derive(GlobalState, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    #[default]
    Loading,
    MainMenu,
    IntroCutscene,
    Overworld,
    TownOutside,
    TownTavern,
    TownMayor,
    TownConcertHall,
}

impl AppState {
    pub fn is_town(&self) -> bool {
        match *self {
            Self::Loading => false,
            Self::MainMenu => false,
            Self::IntroCutscene => false,
            Self::Overworld => false,
            Self::TownOutside => true,
            Self::TownTavern => true,
            Self::TownMayor => true,
            Self::TownConcertHall => true,
        }
    }
}
