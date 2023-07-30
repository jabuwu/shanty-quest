use crate::common::prelude::*;
use crate::game::prelude::*;
use crate::DEV_BUILD;
use bevy::prelude::*;

const CAMERA_SIZE: Vec2 = Vec2::new(1280., 768.);
const WORLD_LIMITS: (Vec2, Vec2) = (Vec2::new(-400., -12000.), Vec2::new(12000., 400.));

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum OverworldCameraSystem {
    Update,
}

pub struct OverworldCameraPlugin;

impl Plugin for OverworldCameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<OverworldCamera>().add_systems(
            Update,
            overworld_camera_update.in_set(OverworldCameraSystem::Update),
        );
    }
}

#[derive(Default, Resource)]
pub struct OverworldCamera {
    arena: (Vec2, Vec2),
    arena_enabled: bool,
    arena_focus: f32,
    zoom_out: bool,
    entity_focus: Option<Entity>,
    entity_focus_amount: f32,
    entity_last_position: Vec2,
    entity_focus_frames: u32,
    screen_shake: f32,
}

impl OverworldCamera {
    pub fn reset(&mut self) {
        self.arena_enabled = false;
        self.entity_focus = None;
    }

    pub fn arena_disable(&mut self) {
        self.arena_enabled = false;
    }

    pub fn arena_enable(&mut self, position: Vec2, size: Vec2) {
        self.arena = (position, (size - CAMERA_SIZE).max(Vec2::ZERO));
        self.arena_enabled = true;
    }

    pub fn is_arena_enabled(&self) -> bool {
        self.arena_enabled
    }

    pub fn arena(&self) -> Option<(Vec2, Vec2)> {
        if self.arena_enabled {
            Some(self.arena)
        } else {
            None
        }
    }

    pub fn entity_focus(&mut self, entity: Entity) {
        self.entity_focus = Some(entity);
        self.entity_focus_frames = 0;
    }

    pub fn arena_correction(&self, translation: Vec2) -> Option<Vec2> {
        if self.arena_enabled {
            let mut correction = Vec2::ZERO;
            let screen_left = self.arena.0.x - self.arena.1.x - CAMERA_SIZE.x * 0.5 + 150.;
            let screen_right = self.arena.0.x + self.arena.1.x + CAMERA_SIZE.x * 0.5 - 150.;
            let screen_bottom = self.arena.0.y - self.arena.1.y - CAMERA_SIZE.y * 0.5 + 150.;
            let screen_top = self.arena.0.y + self.arena.1.y + CAMERA_SIZE.y * 0.5 - 150.;
            if translation.x < screen_left {
                correction.x = screen_left - translation.x;
            }
            if translation.x > screen_right {
                correction.x = screen_right - translation.x;
            }
            if translation.y < screen_bottom {
                correction.y = screen_bottom - translation.y;
            }
            if translation.y > screen_top {
                correction.y = screen_top - translation.y;
            }
            Some(correction * 0.5)
        } else {
            None
        }
    }

    pub fn screen_shake(&mut self, amt: f32) {
        self.screen_shake += amt;
    }
}

fn overworld_camera_update(
    player_query: Query<Entity, With<Player>>,
    camera_query: Query<Entity, With<Camera>>,
    mut transform_query: Query<&mut Transform2>,
    mut overworld_camera: ResMut<OverworldCamera>,
    global_transform_query: Query<&GlobalTransform>,
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
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
    if DEV_BUILD {
        if input.just_pressed(KeyCode::F3) {
            overworld_camera.zoom_out = !overworld_camera.zoom_out;
        }
    }
    if let Some(player_position) = player_position {
        let mut position = player_position;
        if overworld_camera.arena_enabled {
            overworld_camera.arena_focus += time.delta_seconds();
        } else {
            overworld_camera.arena_focus -= time.delta_seconds();
        }
        if overworld_camera.entity_focus.is_some() {
            overworld_camera.entity_focus_amount += time.delta_seconds();
        } else {
            overworld_camera.entity_focus_amount -= time.delta_seconds();
        }
        overworld_camera.entity_focus_frames += 1;
        let entity_position = if let Some(entity_focus) = overworld_camera.entity_focus {
            if let Ok(transform) = global_transform_query.get(entity_focus) {
                overworld_camera.entity_last_position = transform.translation().truncate();
                transform.translation().truncate()
            } else {
                if overworld_camera.entity_focus_frames > 2 {
                    overworld_camera.entity_focus = None;
                }
                overworld_camera.entity_last_position
            }
        } else {
            overworld_camera.entity_last_position
        };
        overworld_camera.arena_focus = overworld_camera.arena_focus.clamp(0., 1.);
        overworld_camera.entity_focus_amount = overworld_camera.entity_focus_amount.clamp(0., 1.);
        position = position.lerp(
            entity_position,
            ease(Easing::SineInOut, overworld_camera.entity_focus_amount) * 0.25,
        );
        let clamped_position = position.clamp(
            overworld_camera.arena.0 - overworld_camera.arena.1,
            overworld_camera.arena.0 + overworld_camera.arena.1,
        );
        position = position.lerp(
            clamped_position,
            ease(Easing::SineInOut, overworld_camera.arena_focus),
        );
        position = position.clamp(WORLD_LIMITS.0, WORLD_LIMITS.1);
        position += Vec2::new(
            rand::random::<f32>() * 2. - 1.,
            rand::random::<f32>() * 2. - 1.,
        ) * overworld_camera.screen_shake
            * 10.;
        overworld_camera.screen_shake *= 0.000001_f32.powf(time.delta_seconds());
        for camera_entity in camera_query.iter() {
            if let Ok(mut camera_transform) = transform_query.get_mut(camera_entity) {
                camera_transform.translation = position;
                camera_transform.scale = Vec2::ONE;
                if DEV_BUILD && overworld_camera.zoom_out {
                    camera_transform.scale = Vec2::ONE * 3.;
                }
            }
        }
    }
}
