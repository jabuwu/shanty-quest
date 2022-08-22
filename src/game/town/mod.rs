use bevy::prelude::*;

pub struct TownPlugin;

impl Plugin for TownPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(outside::OutsidePlugin)
            .add_plugin(concert_hall::ConcertHallPlugin)
            .add_plugin(mayor::MayorPlugin)
            .add_plugin(tavern::TavernPlugin);
    }
}

pub mod concert_hall;
pub mod mayor;
pub mod outside;
pub mod tavern;
