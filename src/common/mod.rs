use crate::{common::prelude::*, game::prelude::BoatSystem};
use audio_plus::AudioPlusPlugin;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use global_state::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum CommonSystem {
    SafeToStateChange,
}

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            EguiPlugin,
            AudioPlusPlugin,
            menu_bar::MenuBarPlugin,
            mouse::MousePlugin,
            assets::AssetsPlugin,
            transform2::Transform2Plugin,
            ldtk::LdtkPlugin,
            entity_debug::EntityDebugPlugin,
            y_depth::YDepthPlugin,
            collision::CollisionPlugin,
            time_to_live::TimeToLivePlugin,
            clickable::ClickablePlugin,
            screen_fade::ScreenFadePlugin,
            cutscene::CutscenePlugin,
            dialogue::DialoguePlugin,
        ))
        .add_plugins((
            follow_camera::FollowCameraPlugin,
            world_locations::WorldLocationsPlugin,
            map_builder::MapBuilderPlugin,
            force_camera_ratio::ForceRatioPlugin,
            wasm::WasmPlugin,
            volume_control::VolumeControlPlugin,
        ))
        .add_global_state::<app_state::AppState>()
        .init_resource::<asset_library::AssetLibrary>()
        .add_systems(PreUpdate, nan_fix)
        .add_systems(Update, safe_to_state_change.after(BoatSystem::Spawn));
    }
}

fn nan_fix(mut query: Query<&mut Transform2>) {
    for mut transform in query.iter_mut() {
        if !transform.translation.is_finite() {
            transform.translation = Vec2::new(800., -350.);
        }
    }
}

fn safe_to_state_change() {}

pub mod app_state;
pub mod asset_library;
pub mod assets;
pub mod clickable;
pub mod collision;
pub mod cutscene;
pub mod depth_layers;
pub mod dialogue;
pub mod easing;
pub mod entity_debug;
pub mod facing;
pub mod follow_camera;
pub mod force_camera_ratio;
pub mod label;
pub mod ldtk;
pub mod map_builder;
pub mod math;
pub mod menu_bar;
pub mod mouse;
pub mod prelude;
pub mod screen_fade;
pub mod sound_effects;
pub mod time_to_live;
pub mod timed_chance;
pub mod transform2;
pub mod volume_control;
pub mod wasm;
pub mod world_locations;
pub mod y_depth;
