use bevy::prelude::*;
use std::f32::consts::{PI, TAU};

const NORMALIZED_DIAGONAL: f32 = 0.70710677;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Facing {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Facing {
    pub fn from_vec(direction: Vec2) -> Option<Self> {
        if direction.length_squared() > 0. {
            let angle = ((direction.angle_between(Vec2::X) + PI + PI * (1. / 8.)) % TAU) / TAU;
            if angle < 1. / 8. {
                Some(Self::West)
            } else if angle < 2. / 8. {
                Some(Self::NorthWest)
            } else if angle < 3. / 8. {
                Some(Self::North)
            } else if angle < 4. / 8. {
                Some(Self::NorthEast)
            } else if angle < 5. / 8. {
                Some(Self::East)
            } else if angle < 6. / 8. {
                Some(Self::SouthEast)
            } else if angle < 7. / 8. {
                Some(Self::South)
            } else {
                Some(Self::SouthWest)
            }
        } else {
            None
        }
    }

    pub fn to_vec(&self) -> Vec2 {
        match *self {
            Facing::North => Vec2::new(0., 1.),
            Facing::NorthEast => Vec2::new(NORMALIZED_DIAGONAL, NORMALIZED_DIAGONAL),
            Facing::East => Vec2::new(1., 0.),
            Facing::SouthEast => Vec2::new(NORMALIZED_DIAGONAL, -NORMALIZED_DIAGONAL),
            Facing::South => Vec2::new(0., -1.),
            Facing::SouthWest => Vec2::new(-NORMALIZED_DIAGONAL, -NORMALIZED_DIAGONAL),
            Facing::West => Vec2::new(-1., 0.),
            Facing::NorthWest => Vec2::new(-NORMALIZED_DIAGONAL, NORMALIZED_DIAGONAL),
        }
    }
}
