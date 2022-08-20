use crate::game::prelude::*;

#[derive(Default)]
pub struct GameState {
    pub goto_town: Option<TownData>,
}
