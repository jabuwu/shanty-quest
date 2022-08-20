use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    utils::BoxedFuture,
};
use serde::Deserialize;

pub struct LdtkAssetPlugin;

impl Plugin for LdtkAssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<LdtkAsset>()
            .init_asset_loader::<LdtkAssetLoader>();
    }
}

#[derive(Debug, Deserialize, TypeUuid)]
#[uuid = "b953a81b-1523-4a1a-a99d-5db6ccdd1533"]
pub struct LdtkAsset {
    pub map: ldtk2::Ldtk,
}

#[derive(Default)]
pub struct LdtkAssetLoader;

impl AssetLoader for LdtkAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let map = ldtk2::Ldtk::from_str(std::str::from_utf8(bytes)?)?;
            load_context.set_default_asset(LoadedAsset::new(LdtkAsset { map }));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ldtk"]
    }
}
