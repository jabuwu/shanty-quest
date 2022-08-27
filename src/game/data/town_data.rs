use crate::common::prelude::*;
use bevy::prelude::*;

pub const TOWN_NAMES: [&str; 15] = [
    "Portallica",
    "Rolling Rock",
    "Port Floyd",
    "Iron Maiden's Cove",
    "Rocktuga",
    "Isla de la Solo",
    "Sing-a-Rock",
    "Port Sabbath",
    "Saint Pantera",
    "Judas Priest's Isle",
    "Isla de Dio",
    "Republic of Roll",
    "Queen's Cove",
    "Drummer's Isle",
    "Isla la Chorus",
];

pub fn town_safe_name(input: &str) -> String {
    input.replace(" ", "_").replace("'", "_")
}

#[derive(Clone, Debug)]
pub struct TownData {
    pub name: String,
    pub position: Vec2,
    pub spawn_offset: Vec2,
}

impl Default for TownData {
    fn default() -> Self {
        Self {
            name: "Dummy Town".to_owned(),
            position: Vec2::new(800., -350.),
            spawn_offset: Vec2::new(0., -200.),
        }
    }
}

impl TownData {
    pub fn build(name: &str, world_locations: &WorldLocations) -> Self {
        let town_name = town_safe_name(name);
        let position = world_locations.get_single_position(&town_name);
        Self {
            name: String::from(name),
            position,
            spawn_offset: Vec2::new(0., -300.),
        }
    }
}
