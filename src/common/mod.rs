use app_state::prelude::*;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            .add_plugin(menu_bar::MenuBarPlugin)
            .add_plugin(mouse::MousePlugin)
            .add_app_state::<game_state::GameState>()
            .init_resource::<asset_library::AssetLibrary>();
    }
}

pub mod asset_library;
pub mod game_state;
pub mod menu_bar;
pub mod mouse;
pub mod prelude;
