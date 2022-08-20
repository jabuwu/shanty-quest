use asset_struct::AssetStruct;
use bevy::prelude::*;

#[derive(Default, AssetStruct)]
pub struct AssetLibrary {
    #[asset("fonts/FiraSans-Bold.ttf")]
    pub font_default: Handle<Font>,
}
