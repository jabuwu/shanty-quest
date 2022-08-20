use bevy::prelude::*;

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
            let angle = direction.angle_between(Vec2::X);
            if angle > std::f32::consts::PI * 4. / 5. {
                Some(Self::West)
            } else if angle > std::f32::consts::PI * 3. / 5. {
                Some(Self::SouthWest)
            } else if angle > std::f32::consts::PI * 2. / 5. {
                Some(Self::South)
            } else if angle > std::f32::consts::PI * 1. / 5. {
                Some(Self::SouthEast)
            } else if angle > -std::f32::consts::PI * 1. / 5. {
                Some(Self::East)
            } else if angle > -std::f32::consts::PI * 2. / 5. {
                Some(Self::NorthEast)
            } else if angle > -std::f32::consts::PI * 3. / 5. {
                Some(Self::North)
            } else if angle > -std::f32::consts::PI * 4. / 5. {
                Some(Self::NorthWest)
            } else {
                Some(Self::West)
            }
        } else {
            None
        }
    }
}
