use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum OverworldCameraSystems {
    Update,
}

pub struct OverworldCameraPlugin;

impl Plugin for OverworldCameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<OverworldCamera>()
            .add_system(overworld_camera_update.label(OverworldCameraSystems::Update));
    }
}

#[derive(Default)]
pub struct OverworldCamera {}

fn overworld_camera_update(
    player_query: Query<Entity, With<Player>>,
    camera_query: Query<Entity, With<Camera>>,
    mut transform_query: Query<&mut Transform2>,
) {
    let player_position = if let Ok(player_entity) = player_query.get_single() {
        if let Ok(player_transform) = transform_query.get(player_entity) {
            Some(player_transform.translation)
        } else {
            None
        }
    } else {
        None
    };
    if let Some(player_position) = player_position {
        for camera_entity in camera_query.iter() {
            if let Ok(mut camera_transform) = transform_query.get_mut(camera_entity) {
                camera_transform.translation = player_position;
            }
        }
    }
}
