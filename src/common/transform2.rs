use crate::common::prelude::*;
use bevy::prelude::*;
use bevy::transform::TransformSystem;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum Transform2System {
    TransformPropagate,
}

pub struct Transform2Plugin;

impl Plugin for Transform2Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            update_transform2
                .in_set(Transform2System::TransformPropagate)
                .before(TransformSystem::TransformPropagate),
        );
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum DepthLayer {
    #[default]
    Inherit,
    Environment,
    Entity,
    Front,
    Debug,
    Camera,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Transform2 {
    pub translation: Vec2,
    pub rotation: f32,
    pub scale: Vec2,
    pub depth_layer: DepthLayer,
    pub depth: f32,
    pub pixel_perfect: bool,
}

impl Default for Transform2 {
    fn default() -> Self {
        Self {
            translation: Vec2::ZERO,
            rotation: 0.0,
            scale: Vec2::ONE,
            depth_layer: DepthLayer::default(),
            depth: 0.0,
            pixel_perfect: true,
        }
    }
}

impl Transform2 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_xy(x: f32, y: f32) -> Self {
        Self {
            translation: Vec2::new(x, y),
            ..Default::default()
        }
    }

    pub fn from_translation(translation: Vec2) -> Self {
        Self {
            translation,
            ..Default::default()
        }
    }

    pub fn with_rotation(self, rotation: f32) -> Self {
        Self {
            rotation: rotation,
            ..self
        }
    }

    pub fn with_depth(self, depth: (DepthLayer, f32)) -> Self {
        Self {
            depth_layer: depth.0,
            depth: depth.1,
            ..self
        }
    }

    pub fn with_scale(self, scale: Vec2) -> Self {
        Self { scale, ..self }
    }

    pub fn without_pixel_perfect(self) -> Self {
        Self {
            pixel_perfect: false,
            ..self
        }
    }

    pub fn depth_f32(&self) -> f32 {
        match self.depth_layer {
            DepthLayer::Inherit => self.depth.lerp(0.0, 0.01),
            DepthLayer::Environment => self.depth.lerp(0.1, 0.29),
            DepthLayer::Entity => self.depth.lerp(0.3, 0.49),
            DepthLayer::Front => self.depth.lerp(0.5, 0.69),
            DepthLayer::Debug => self.depth.lerp(0.7, 0.89),
            DepthLayer::Camera => 1.0,
        }
    }
}

#[derive(Bundle, Clone, Default)]
pub struct Transform2Bundle {
    pub transform2: Transform2,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

fn update_transform2(
    root_query: Query<Entity, Without<Parent>>,
    children_query: Query<&Children>,
    mut transform_query: Query<(&mut Transform, &Transform2)>,
) {
    for root in root_query.iter() {
        update_transform2_recursive(root, &children_query, &mut transform_query, 0.);
    }
}

fn update_transform2_recursive(
    entity: Entity,
    children_query: &Query<&Children>,
    transform_query: &mut Query<(&mut Transform, &Transform2)>,
    mut cumulative_depth: f32,
) {
    if let Some((mut transform, transform2)) = transform_query.get_mut(entity).ok() {
        transform.translation.x = transform2.translation.x;
        transform.translation.y = transform2.translation.y;
        transform.scale = Vec3::new(transform2.scale.x, transform2.scale.y, 1.0);
        transform.rotation = Quat::from_rotation_z(transform2.rotation);
        if transform2.depth_layer == DepthLayer::Inherit {
            transform.translation.z = transform2.depth_f32();
        } else {
            transform.translation.z = transform2.depth_f32() - cumulative_depth;
        }
        cumulative_depth += transform.translation.z;
    }
    if let Some(children) = children_query.get(entity).ok() {
        for child in children.iter() {
            update_transform2_recursive(*child, children_query, transform_query, cumulative_depth);
        }
    }
}
