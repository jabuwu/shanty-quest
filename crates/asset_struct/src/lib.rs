pub use asset_struct_macros::AssetStruct;
use bevy::{asset::Asset, ecs::system::Resource, prelude::*};

pub trait AssetStruct {
    fn load_assets(&mut self, asset_server: &Res<AssetServer>);
    fn load_state(&self, asset_server: &Res<AssetServer>) -> bevy::asset::LoadState;
    fn load_progress(&self, asset_server: &Res<AssetServer>) -> f32;
    fn from_filename<T: Asset>(&self, path: &str) -> Handle<T>;
}

pub trait AddAssetStruct {
    fn add_asset_struct<T: AssetStruct + Resource + Default>(&mut self) -> &mut Self;
}

impl AddAssetStruct for App {
    fn add_asset_struct<T: AssetStruct + Resource + Default>(&mut self) -> &mut Self {
        self.init_resource::<T>()
    }
}

pub mod prelude;
