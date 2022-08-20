use crate::common::prelude::*;
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CollisionShape {
    None,
    Point,
    Rect { size: Vec2 },
}

impl CollisionShape {
    pub fn overlaps(&self, a_position: Vec2, b_shape: CollisionShape, b_position: Vec2) -> bool {
        match *self {
            CollisionShape::None => false,
            CollisionShape::Point => match b_shape {
                CollisionShape::None => false,
                CollisionShape::Point => check_point_point(a_position, b_position),
                CollisionShape::Rect { size: b_size } => {
                    check_point_rect(a_position, b_position, b_size)
                }
            },
            CollisionShape::Rect { size: a_size } => match b_shape {
                CollisionShape::None => false,
                CollisionShape::Point => check_point_rect(b_position, a_position, a_size),
                CollisionShape::Rect { size: b_size } => {
                    check_rect_rect(a_position, a_size, b_position, b_size)
                }
            },
        }
    }

    pub fn overlaps_moving(
        &self,
        a_position: Vec2,
        a_velocity: Vec2,
        b_shape: CollisionShape,
        b_position: Vec2,
        b_velocity: Vec2,
    ) -> Option<f32> {
        match *self {
            CollisionShape::None => None,
            CollisionShape::Point => match b_shape {
                CollisionShape::None => None,
                CollisionShape::Point => None,
                CollisionShape::Rect { .. } => None,
            },
            CollisionShape::Rect { size: a_size } => match b_shape {
                CollisionShape::None => None,
                CollisionShape::Point => None,
                CollisionShape::Rect { size: b_size } => check_rect_moving_rect(
                    a_position,
                    a_size,
                    b_position,
                    b_size,
                    b_velocity - a_velocity,
                ),
            },
        }
    }
}

impl Default for CollisionShape {
    fn default() -> Self {
        Self::None
    }
}

pub fn rect_extents(position: Vec2, size: Vec2) -> Rect<f32> {
    let half_size = size * 0.5;
    Rect {
        left: position.x - half_size.x,
        right: position.x + half_size.x,
        bottom: position.y - half_size.y,
        top: position.y + half_size.y,
    }
}

pub fn check_point_point(a_position: Vec2, b_position: Vec2) -> bool {
    a_position.x == b_position.x && a_position.y == b_position.y
}

pub fn check_point_rect(a_position: Vec2, b_position: Vec2, b_size: Vec2) -> bool {
    let b_rect = rect_extents(b_position, b_size);
    b_rect.left <= a_position.x
        && b_rect.right >= a_position.x
        && b_rect.bottom <= a_position.y
        && b_rect.top >= a_position.y
}

pub fn check_rect_rect(a_position: Vec2, a_size: Vec2, b_position: Vec2, b_size: Vec2) -> bool {
    let a_rect = rect_extents(a_position, a_size);
    let b_rect = rect_extents(b_position, b_size);
    a_rect.left <= b_rect.right
        && a_rect.right >= b_rect.left
        && a_rect.bottom <= b_rect.top
        && a_rect.top >= b_rect.bottom
}

pub fn check_rect_moving_rect(
    a_position: Vec2,
    a_size: Vec2,
    b_position: Vec2,
    b_size: Vec2,
    b_velocity: Vec2,
) -> Option<f32> {
    if check_rect_rect(a_position, a_size, b_position, b_size) {
        Some(0.0)
    } else {
        if b_velocity.length_squared() == 0.0 {
            return None;
        }
        let a_rect = rect_extents(a_position, a_size);
        let b_rect = rect_extents(b_position, b_size);
        let mut t_first: f32 = 0.0;
        let mut t_last: f32 = 1.0;
        if b_velocity.x < 0.0 {
            if b_rect.right < a_rect.left {
                return None;
            }
            if a_rect.right < b_rect.left {
                t_first = t_first.max((a_rect.right - b_rect.left) / b_velocity.x);
            };
            if b_rect.right > a_rect.left {
                t_last = t_last.min((a_rect.left - b_rect.right) / b_velocity.x)
            };
        } else if b_velocity.x > 0.0 {
            if b_rect.left > a_rect.right {
                return None;
            }
            if b_rect.right < a_rect.left {
                t_first = t_first.max((a_rect.left - b_rect.right) / b_velocity.x)
            };
            if a_rect.right > b_rect.left {
                t_last = t_last.min((a_rect.right - b_rect.left) / b_velocity.x)
            };
        } else if a_rect.left > b_rect.right || a_rect.right < b_rect.left {
            return None;
        }
        if t_first > t_last {
            return None;
        }
        if b_velocity.y < 0.0 {
            if b_rect.top < a_rect.bottom {
                return None;
            }
            if a_rect.top < b_rect.bottom {
                t_first = t_first.max((a_rect.top - b_rect.bottom) / b_velocity.y);
            };
            if b_rect.top > a_rect.bottom {
                t_last = t_last.min((a_rect.bottom - b_rect.top) / b_velocity.y)
            };
        } else if b_velocity.y > 0.0 {
            if b_rect.bottom > a_rect.top {
                return None;
            }
            if b_rect.top < a_rect.bottom {
                t_first = t_first.max((a_rect.bottom - b_rect.top) / b_velocity.y)
            };
            if a_rect.top > b_rect.bottom {
                t_last = t_last.min((a_rect.top - b_rect.bottom) / b_velocity.y)
            };
        } else if a_rect.bottom > b_rect.top || a_rect.top < b_rect.bottom {
            return None;
        }
        if t_first > t_last {
            return None;
        }
        if t_first > 1.0 {
            return None;
        }
        Some(t_first)
    }
}
