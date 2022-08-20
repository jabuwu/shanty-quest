use crate::common::prelude::*;
use asset_struct::AssetStruct;
use bevy::prelude::*;

#[derive(Default, AssetStruct)]
pub struct AssetLibrary {
    #[asset("fonts/FiraSans-Bold.ttf")]
    pub font_default: Handle<Font>,

    #[asset("levels/test.ldtk")]
    pub level_test: Handle<LdtkAsset>,
    #[asset("levels/IslandTilemap.png")]
    pub level_test_island_tilemap: Handle<Image>,

    #[asset("levels/WaterTileMap.png")]
    pub level_test_water_tilemap: Handle<Image>,

    #[asset("sprites/Ship1.png")]
    pub sprite_ship: Handle<Image>,
}
