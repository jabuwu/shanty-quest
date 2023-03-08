use bevy::{prelude::*, transform::TransformSystem};
use global_state::Persistent;

use super::transform2::Transform2System;

const DESIRED_SIZE: Vec2 = Vec2::new(1280., 768.);
const RATIO_BAR_SIZE: f32 = 100000.;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum ForceRatioSystem {
    Setup,
    Update,
}

pub struct ForceRatioPlugin;

impl Plugin for ForceRatioPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(force_ratio_setup.in_set(ForceRatioSystem::Setup))
            .add_system(
                force_ratio_update
                    .in_set(ForceRatioSystem::Update)
                    .in_base_set(CoreSet::PostUpdate)
                    .before(TransformSystem::TransformPropagate)
                    .after(Transform2System::TransformPropagate),
            );
    }
}

#[derive(Component, Clone, Copy)]
pub enum ForceRatioBar {
    Top,
    Bottom,
    Left,
    Right,
}

impl ForceRatioBar {
    fn translation(&self) -> Vec3 {
        match *self {
            ForceRatioBar::Top => Vec3::new(0., DESIRED_SIZE.y * 0.5 + RATIO_BAR_SIZE * 0.5, 1.),
            ForceRatioBar::Bottom => {
                Vec3::new(0., DESIRED_SIZE.y * -0.5 - RATIO_BAR_SIZE * 0.5, 1.)
            }
            ForceRatioBar::Left => Vec3::new(DESIRED_SIZE.x * -0.5 - RATIO_BAR_SIZE * 0.5, 0., 1.),
            ForceRatioBar::Right => Vec3::new(DESIRED_SIZE.x * 0.5 + RATIO_BAR_SIZE * 0.5, 0., 1.),
        }
    }
}

fn force_ratio_setup(mut commands: Commands) {
    for side in [
        ForceRatioBar::Top,
        ForceRatioBar::Bottom,
        ForceRatioBar::Left,
        ForceRatioBar::Right,
    ] {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(RATIO_BAR_SIZE)),
                    color: Color::BLACK,
                    ..Default::default()
                },
                transform: Transform::from_translation(side.translation()),
                ..Default::default()
            },
            Persistent,
            side,
        ));
    }
}

fn force_ratio_update(
    mut transform_query: Query<&mut Transform>,
    camera_query: Query<Entity, With<Camera>>,
    bar_query: Query<(Entity, &ForceRatioBar)>,
    window_query: Query<&Window>,
) {
    let mut camera_position = Vec3::ZERO;
    if let Some(window) = window_query.get_single().ok() {
        for camera_entity in camera_query.iter() {
            if let Some(mut camera_transform) = transform_query.get_mut(camera_entity).ok() {
                let ratio = window.width() / window.height();
                let mut desired_width = DESIRED_SIZE.x;
                let mut desired_height = DESIRED_SIZE.y;
                let desired_ratio = desired_width / desired_height;
                if ratio > desired_ratio {
                    desired_width *= ratio / desired_ratio;
                } else {
                    desired_height *= desired_ratio / ratio;
                }
                camera_transform.scale.x = desired_width / window.width();
                camera_transform.scale.y = desired_height / window.height();
                camera_position = camera_transform.translation.truncate().extend(0.);
            }
        }
    }
    for (bar_entity, bar) in bar_query.iter() {
        if let Some(mut bar_transform) = transform_query.get_mut(bar_entity).ok() {
            bar_transform.translation = camera_position + bar.translation();
        }
    }
}
