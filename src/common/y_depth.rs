use crate::common::prelude::*;
use bevy::prelude::*;

pub struct YDepthPlugin;

impl Plugin for YDepthPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(y_depth);
    }
}

#[derive(Default, Component)]
pub struct YDepth;

pub fn y_depth(mut query: Query<&mut Transform2, With<YDepth>>) {
    for mut transform in query.iter_mut() {
        transform.depth = 0.5 - transform.translation.y / 10000.;
    }
}
