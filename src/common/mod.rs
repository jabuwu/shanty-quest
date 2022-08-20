use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use global_state::prelude::*;

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            .add_plugin(menu_bar::MenuBarPlugin)
            .add_plugin(mouse::MousePlugin)
            .add_plugin(assets::AssetsPlugin)
            .add_plugin(transform2::Transform2Plugin)
            .add_plugin(ldtk::LdtkPlugin)
            .add_plugin(entity_debug::EntityDebugPlugin)
            .add_plugin(y_depth::YDepthPlugin)
            .add_global_state::<app_state::AppState>()
            .init_resource::<asset_library::AssetLibrary>();
    }
}

pub mod app_state;
pub mod asset_library;
pub mod assets;
pub mod entity_debug;
pub mod facing;
pub mod game_math;
pub mod label;
pub mod ldtk;
pub mod menu_bar;
pub mod mouse;
pub mod prelude;
pub mod transform2;
pub mod y_depth;
