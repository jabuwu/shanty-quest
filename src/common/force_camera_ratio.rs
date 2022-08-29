use bevy::{prelude::*, transform::TransformSystem};

use super::transform2::Transform2System;

pub struct ForceCameraRatioPlugin;

impl Plugin for ForceCameraRatioPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(
            CoreStage::PostUpdate,
            force_camera_ratio
                .after(Transform2System::TransformPropagate)
                .before(TransformSystem::TransformPropagate),
        );
    }
}

fn force_camera_ratio(windows: Res<Windows>, mut query: Query<&mut Transform, With<Camera>>) {
    if let Some(window) = windows.get_primary() {
        for mut transform in query.iter_mut() {
            transform.scale.x = 1280. / window.width();
            transform.scale.y = 768. / window.height();
        }
    }
}
