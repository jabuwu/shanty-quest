use crate::common::prelude::*;

type Layer = (DepthLayer, f32);

// environment
pub const DEPTH_LAYER_OCEAN: Layer = (DepthLayer::Environment, 0.0);
pub const DEPTH_LAYER_OCEAN_OVERLAY: Layer = (DepthLayer::Environment, 0.01);
pub const DEPTH_LAYER_BOAT_TRAIL: Layer = (DepthLayer::Environment, 0.015);
pub const DEPTH_LAYER_SHOCKWAVE: Layer = (DepthLayer::Environment, 0.02);
pub const DEPTH_LAYER_CONTROLS: Layer = (DepthLayer::Environment, 0.1);

// ui
pub const DEPTH_LAYER_SCREEN_FADE: Layer = (DepthLayer::Front, 1.);

pub const DEPTH_LAYER_DIALOGUE_BACK: Layer = (DepthLayer::Front, 0.95);
pub const DEPTH_LAYER_DIALOGUE_TEXT: Layer = (DepthLayer::Front, 0.96);
pub const DEPTH_LAYER_DIALOGUE_PORTRAIT: Layer = (DepthLayer::Front, 0.94);

pub const DEPTH_LAYER_HEALTHBAR_BORDER: Layer = (DepthLayer::Front, 0.1);
pub const DEPTH_LAYER_HEALTHBAR: Layer = (DepthLayer::Front, 0.11);

pub const DEPTH_LAYER_TOWN_NAME: Layer = (DepthLayer::Front, 0.);

pub const DEPTH_LAYER_MAP_BACK: Layer = (DepthLayer::Front, 0.90);
pub const DEPTH_LAYER_MAP_TILE: Layer = (DepthLayer::Front, 0.91);
pub const DEPTH_LAYER_MAP_PLAYER: Layer = (DepthLayer::Front, 0.92);
pub const DEPTH_LAYER_MAP_LABEL: Layer = (DepthLayer::Front, 0.93);

pub const DEPTH_LAYER_UI_TEXT: Layer = (DepthLayer::Front, 0.85);
pub const DEPTH_LAYER_UI_MARKER_ICON: Layer = (DepthLayer::Front, 0.86);
pub const DEPTH_LAYER_UI_MARKER_ARROW: Layer = (DepthLayer::Front, 0.87);

pub const DEPTH_LAYER_BAND_SELECTION_BACK: Layer = (DepthLayer::Front, 0.5);
pub const DEPTH_LAYER_BAND_SELECTION_SLOT: Layer = (DepthLayer::Front, 0.51);
pub const DEPTH_LAYER_BAND_SELECTION_SLOT_RAISED: Layer = (DepthLayer::Front, 0.52);

pub const DEPTH_LAYER_TOWN_OUTSIDE_BG: Layer = (DepthLayer::Front, 0.0);
pub const DEPTH_LAYER_TOWN_OUTSIDE_HIGHLIGHT: Layer = (DepthLayer::Front, 0.01);
pub const DEPTH_LAYER_TOWN_OUTSIDE_EXIT: Layer = (DepthLayer::Front, 0.01);
pub const DEPTH_LAYER_TOWN_OUTSIDE_NAME: Layer = (DepthLayer::Front, 0.01);
