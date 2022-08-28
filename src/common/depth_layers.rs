use crate::common::prelude::*;

type Layer = (DepthLayer, f32);

// environment
pub const DEPTH_LAYER_OCEAN: Layer = (DepthLayer::Environment, 0.0);
pub const DEPTH_LAYER_OCEAN_OVERLAY: Layer = (DepthLayer::Environment, 0.01);
pub const DEPTH_LAYER_BOAT_TRAIL: Layer = (DepthLayer::Environment, 0.015);
pub const DEPTH_LAYER_SHOCKWAVE: Layer = (DepthLayer::Environment, 0.02);
pub const DEPTH_LAYER_CONTROLS: Layer = (DepthLayer::Environment, 0.1);
pub const DEPTH_LAYER_EXPERIENCE: Layer = (DepthLayer::Environment, 0.11);

// ui
pub const DEPTH_LAYER_SCREEN_FADE: Layer = (DepthLayer::Front, 1.);
pub const DEPTH_LAYER_DAMAGE_FLASH: Layer = (DepthLayer::Front, 0.99);

pub const DEPTH_LAYER_DIALOGUE_FADE: Layer = (DepthLayer::Front, 0.94);
pub const DEPTH_LAYER_DIALOGUE_BACK: Layer = (DepthLayer::Front, 0.95);
pub const DEPTH_LAYER_DIALOGUE_TEXT: Layer = (DepthLayer::Front, 0.96);
pub const DEPTH_LAYER_DIALOGUE_PORTRAIT: Layer = (DepthLayer::Front, 0.94);

pub const DEPTH_LAYER_HEALTHBAR_BORDER: Layer = (DepthLayer::Front, 0.1);
pub const DEPTH_LAYER_HEALTHBAR: Layer = (DepthLayer::Front, 0.11);

pub const DEPTH_LAYER_TOWN_NAME: Layer = (DepthLayer::Front, 0.);

pub const DEPTH_LAYER_MAP_BACK: Layer = (DepthLayer::Front, 0.95);
pub const DEPTH_LAYER_MAP_BACK_COLOR: Layer = (DepthLayer::Front, 0.951);
pub const DEPTH_LAYER_MAP_TILE: Layer = (DepthLayer::Front, 0.952);
pub const DEPTH_LAYER_MAP_PLAYER: Layer = (DepthLayer::Front, 0.954);
pub const DEPTH_LAYER_MAP_OBJECTIVE: Layer = (DepthLayer::Front, 0.953);
pub const DEPTH_LAYER_MAP_LABEL: Layer = (DepthLayer::Front, 0.955);
pub const DEPTH_LAYER_MAP_COMPASS: Layer = (DepthLayer::Front, 0.956);

pub const DEPTH_LAYER_CHECKPOINT_BACKGROUND: Layer = (DepthLayer::Front, 0.96);
pub const DEPTH_LAYER_CHECKPOINT_TEXT: Layer = (DepthLayer::Front, 0.961);
pub const DEPTH_LAYER_LEVEL_UP_BACKGROUND: Layer = (DepthLayer::Front, 0.97);
pub const DEPTH_LAYER_LEVEL_UP_TEXT: Layer = (DepthLayer::Front, 0.971);

pub const DEPTH_LAYER_UI_TEXT: Layer = (DepthLayer::Front, 0.85);
pub const DEPTH_LAYER_UI_MARKER_ICON: Layer = (DepthLayer::Front, 0.851);
pub const DEPTH_LAYER_UI_MARKER_ARROW: Layer = (DepthLayer::Front, 0.852);
pub const DEPTH_LAYER_UI_OBJECTIVE_BACKGROUND: Layer = (DepthLayer::Front, 0.86);
pub const DEPTH_LAYER_UI_OBJECTIVE_TEXT: Layer = (DepthLayer::Front, 0.861);
pub const DEPTH_LAYER_UI_BOSS_HEALTHBAR_NAME_BACKGROUND: Layer = (DepthLayer::Front, 0.86);
pub const DEPTH_LAYER_UI_BOSS_HEALTHBAR_NAME: Layer = (DepthLayer::Front, 0.861);
pub const DEPTH_LAYER_UI_BOSS_HEALTHBAR_BORDER: Layer = (DepthLayer::Front, 0.86);
pub const DEPTH_LAYER_UI_BOSS_HEALTHBAR: Layer = (DepthLayer::Front, 0.861);
pub const DEPTH_LAYER_UI_HEALTH_BOTTLE: Layer = (DepthLayer::Front, 0.905);
pub const DEPTH_LAYER_UI_CONTROLS: Layer = (DepthLayer::Front, 0.905);
pub const DEPTH_LAYER_UI_CONTROLS_KEY: Layer = (DepthLayer::Front, 0.906);
pub const DEPTH_LAYER_UI_EXPERIENCE_BAR_BACK: Layer = (DepthLayer::Front, 0.905);
pub const DEPTH_LAYER_UI_EXPERIENCE_BAR: Layer = (DepthLayer::Front, 0.906);
pub const DEPTH_LAYER_UI_EXPERIENCE_LEVEL: Layer = (DepthLayer::Front, 0.907);
pub const DEPTH_LAYER_UI_EXPERIENCE_SKILLPOINT_BG: Layer = (DepthLayer::Front, 0.907);
pub const DEPTH_LAYER_UI_EXPERIENCE_SKILLPOINT_TEXT: Layer = (DepthLayer::Front, 0.908);

pub const DEPTH_LAYER_BAND_SELECTION_BACK: Layer = (DepthLayer::Front, 0.5);
pub const DEPTH_LAYER_BAND_SELECTION_SLOT: Layer = (DepthLayer::Front, 0.51);
pub const DEPTH_LAYER_BAND_SELECTION_SLOT_RAISED: Layer = (DepthLayer::Front, 0.52);

pub const DEPTH_LAYER_TOWN_OUTSIDE_BG: Layer = (DepthLayer::Front, 0.0);
pub const DEPTH_LAYER_TOWN_OUTSIDE_HIGHLIGHT: Layer = (DepthLayer::Front, 0.01);
pub const DEPTH_LAYER_TOWN_OUTSIDE_EXIT: Layer = (DepthLayer::Front, 0.01);
pub const DEPTH_LAYER_TOWN_OUTSIDE_NAME: Layer = (DepthLayer::Front, 0.01);
pub const DEPTH_LAYER_TOWN_OUTSIDE_ICON: Layer = (DepthLayer::Front, 0.02);

pub const DEPTH_LAYER_TOWN_OUTSIDE_RUM_REFILL_BG: Layer = (DepthLayer::Front, 0.1);
pub const DEPTH_LAYER_TOWN_OUTSIDE_RUM_REFILL_BOTTLE: Layer = (DepthLayer::Front, 0.11);

pub const DEPTH_LAYER_UPGRADES_LEAVE_TEXT: Layer = (DepthLayer::Front, 0.5);
pub const DEPTH_LAYER_UPGRADES_BG: Layer = (DepthLayer::Front, 0.5);
pub const DEPTH_LAYER_UPGRADES_SKILLPOINT: Layer = (DepthLayer::Front, 0.501);
pub const DEPTH_LAYER_UPGRADES_ABILITY_BG: Layer = (DepthLayer::Front, 0.501);
pub const DEPTH_LAYER_UPGRADES_ABILITY_ICON: Layer = (DepthLayer::Front, 0.502);
pub const DEPTH_LAYER_UPGRADES_ABILITY_TEXT: Layer = (DepthLayer::Front, 0.502);
pub const DEPTH_LAYER_UPGRADES_ABILITY_BUTTON: Layer = (DepthLayer::Front, 0.502);
pub const DEPTH_LAYER_UPGRADES_STAR: Layer = (DepthLayer::Front, 0.502);
