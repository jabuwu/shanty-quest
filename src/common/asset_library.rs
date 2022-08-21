use crate::common::prelude::*;
use asset_struct::AssetStruct;
use bevy::prelude::*;
use bevy_kira_audio::AudioSource;

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
    pub sprite_ship_atlas: Handle<TextureAtlas>,

    #[asset("sprites/WaterOverlay.png")]
    pub sprite_water_overlay: Handle<Image>,
    #[asset("sprites/WaterOverlay2.png")]
    pub sprite_water_overlay2: Handle<Image>,

    #[asset("sprites/WaterRingVFX.png")]
    pub sprite_water_ring_vfx: Handle<Image>,

    #[asset("music/ye_old_pirate_rave.ogg")]
    pub music_pirate_rave: Handle<AudioSource>,
}

impl AssetLibrary {
    pub fn create_texture_atlases(&mut self, texture_atlas_assets: &mut Assets<TextureAtlas>) {
        let texture_atlas =
            TextureAtlas::from_grid(self.sprite_ship.clone(), Vec2::new(250., 350.), 5, 1);
        self.sprite_ship_atlas = texture_atlas_assets.add(texture_atlas);
    }
}
