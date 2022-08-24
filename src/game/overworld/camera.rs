use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;

const CAMERA_SIZE: Vec2 = Vec2::new(1280., 768.);
const WORLD_LIMITS: (Vec2, Vec2) = (Vec2::new(-400., -90000.), Vec2::new(90000., 400.));

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
pub struct OverworldCamera {
    arena: (Vec2, Vec2),
    arena_enabled: bool,
    arena_focus: f32,
}

impl OverworldCamera {
    pub fn reset(&mut self) {
        self.arena_enabled = false;
    }

    pub fn disable_arena(&mut self) {
        self.arena_enabled = false;
    }

    pub fn enable_arena(&mut self, position: Vec2, size: Vec2) {
        self.arena = (position, size.min(CAMERA_SIZE * 0.5 * 0.25));
        self.arena_enabled = true;
    }
}

fn overworld_camera_update(
    player_query: Query<Entity, With<Player>>,
    camera_query: Query<Entity, With<Camera>>,
    mut transform_query: Query<&mut Transform2>,
    mut overworld_camera: ResMut<OverworldCamera>,
    time: Res<Time>,
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
        let mut position = player_position;
        if overworld_camera.arena_enabled {
            overworld_camera.arena_focus += time.delta_seconds();
        } else {
            overworld_camera.arena_focus -= time.delta_seconds();
        }
        overworld_camera.arena_focus = overworld_camera.arena_focus.clamp(0., 1.);
        let clamped_position = position.clamp(
            overworld_camera.arena.0 - overworld_camera.arena.1,
            overworld_camera.arena.0 + overworld_camera.arena.1,
        );
        position = position.lerp(
            clamped_position,
            ease(Easing::SineInOut, overworld_camera.arena_focus),
        );
        position = position.clamp(WORLD_LIMITS.0, WORLD_LIMITS.1);
        for camera_entity in camera_query.iter() {
            if let Ok(mut camera_transform) = transform_query.get_mut(camera_entity) {
                camera_transform.translation = position;
            }
        }
    }
}
