use crate::common::prelude::*;

type Layer = (DepthLayer, f32);

// environment
pub const DEPTH_LAYER_OCEAN: Layer = (DepthLayer::Environment, 0.0);
pub const DEPTH_LAYER_OCEAN_OVERLAY: Layer = (DepthLayer::Environment, 0.01);
pub const DEPTH_LAYER_BOAT_TRAIL: Layer = (DepthLayer::Environment, 0.015);
pub const DEPTH_LAYER_SHOCKWAVE: Layer = (DepthLayer::Environment, 0.02);
pub const DEPTH_LAYER_CONTROLS: Layer = (DepthLayer::Environment, 0.1);
