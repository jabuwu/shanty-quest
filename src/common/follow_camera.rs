use super::transform2::Transform2System;
use crate::common::prelude::*;
use bevy::prelude::*;

pub struct FollowCameraPlugin;

impl Plugin for FollowCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            follow_camera
                .in_base_set(CoreSet::PostUpdate)
                .before(Transform2System::TransformPropagate),
        );
    }
}

#[derive(Component)]
pub struct FollowCamera {
    pub offset: Vec2,
}

fn follow_camera(
    query: Query<(Entity, &FollowCamera)>,
    camera_query: Query<Entity, With<Camera>>,
    mut transform_query: Query<&mut Transform2>,
) {
    let camera_entity = if let Ok(camera_entity) = camera_query.get_single() {
        camera_entity
    } else {
        return;
    };
    let camera_translation = if let Ok(camera_transform) = transform_query.get(camera_entity) {
        camera_transform.translation
    } else {
        return;
    };
    for (follow_entity, follow_camera) in query.iter() {
        if let Ok(mut transform) = transform_query.get_mut(follow_entity) {
            transform.translation.x = camera_translation.x + follow_camera.offset.x;
            transform.translation.y = camera_translation.y + follow_camera.offset.y;

            transform.translation.x = transform.translation.x.round();
            transform.translation.y = transform.translation.y.round();
        }
    }
}
