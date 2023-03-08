use bevy::{prelude::*, transform::TransformSystem};

use super::transform2::Transform2Set;

pub struct ForceCameraRatioPlugin;

impl Plugin for ForceCameraRatioPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            force_camera_ratio
                .in_base_set(CoreSet::PostUpdate)
                .after(Transform2Set::TransformPropagate)
                .before(TransformSystem::TransformPropagate),
        );
    }
}

fn force_camera_ratio(
    window_query: Query<&Window>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    if let Some(window) = window_query.get_single().ok() {
        for mut transform in query.iter_mut() {
            transform.scale.x = 1280. / window.width();
            transform.scale.y = 768. / window.height();
        }
    }
}
