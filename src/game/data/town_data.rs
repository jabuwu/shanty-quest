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
