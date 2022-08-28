use bevy::prelude::*;
use bevy::transform::TransformSystem;
use shape::CollisionShape;

pub const COLLISION_FLAG: u32 = 1;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CollisionQuery>().add_system_to_stage(
            CoreStage::PostUpdate,
            update_collision_query.before(TransformSystem::TransformPropagate),
        );
    }
}

#[derive(Component, Default)]
pub struct Collision {
    pub shape: CollisionShape,
    pub flags: u32,
}

pub struct CollisionQueryEntry {
    pub entity: Entity,
    pub position: Vec2,
    pub shape: CollisionShape,
    pub flags: u32,
}

#[derive(Default)]
pub struct CollisionQuery {
    entries: Vec<CollisionQueryEntry>,
}

#[derive(Copy, Clone)]
pub struct CollisionFilter {
    pub exclude_entity: Entity,
    pub flags: u32,
}

impl CollisionQuery {
    pub fn check(
        &self,
        position: Vec2,
        shape: CollisionShape,
        filter: Option<CollisionFilter>,
    ) -> Option<(Entity, Vec2)> {
        for entry in self.entries.iter() {
            if shape.overlaps(position, entry.shape, entry.position) {
                if let Some(ref filter) = filter {
                    if filter.exclude_entity != entry.entity && filter.flags & entry.flags != 0 {
                        return Some((entry.entity, entry.position - position));
                    }
                } else {
                    return Some((entry.entity, entry.position - position));
                }
            }
        }
        None
    }

    pub fn check_moving(
        &self,
        position: Vec2,
        velocity: Vec2,
        shape: CollisionShape,
        filter: Option<CollisionFilter>,
    ) -> Option<(Entity, f32)> {
        let mut result: Option<(Entity, f32)> = None;
        for entry in self.entries.iter() {
            if let Some(collide_time) =
                shape.overlaps_moving(position, velocity, entry.shape, entry.position, Vec2::ZERO)
            {
                if let Some(ref filter) = filter {
                    if filter.exclude_entity != entry.entity && filter.flags & entry.flags != 0 {
                        if let Some((_, other_collide_time)) = result {
                            if collide_time < other_collide_time {
                                result = Some((entry.entity, collide_time));
                            }
                        } else {
                            result = Some((entry.entity, collide_time));
                        }
                    }
                } else {
                    if let Some((_, other_collide_time)) = result {
                        if collide_time < other_collide_time {
                            result = Some((entry.entity, collide_time));
                        }
                    } else {
                        result = Some((entry.entity, collide_time));
                    }
                }
            }
        }
        result
    }

    pub fn check_all(
        &self,
        position: Vec2,
        shape: CollisionShape,
        filter: Option<CollisionFilter>,
    ) -> Vec<Entity> {
        let mut vec: Vec<Entity> = vec![];
        for entry in self.entries.iter() {
            if shape.overlaps(position, entry.shape, entry.position) {
                if let Some(ref filter) = filter {
                    if filter.exclude_entity != entry.entity && filter.flags & entry.flags != 0 {
                        vec.push(entry.entity);
                    }
                } else {
                    vec.push(entry.entity);
                }
            }
        }
        vec
    }

    pub fn update(&mut self, query: &Query<(Entity, &GlobalTransform, &Collision)>) {
        self.entries.clear();
        for (entity, transform, collision) in query.iter() {
            if transform.translation().is_finite() {
                self.entries.push(CollisionQueryEntry {
                    entity,
                    position: transform.translation().truncate(),
                    shape: collision.shape,
                    flags: collision.flags,
                });
            }
        }
    }
}

fn update_collision_query(
    mut collision_query: ResMut<CollisionQuery>,
    query: Query<(Entity, &GlobalTransform, &Collision)>,
) {
    collision_query.update(&query);
}

pub mod shape;
