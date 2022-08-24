use bevy::{prelude::*, utils::HashMap};

pub struct WorldLocationsPlugin;

impl Plugin for WorldLocationsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldLocations>()
            .add_event::<WorldLocationsSpawnEvent>();
    }
}

#[derive(Default, Clone)]
pub struct WorldLocationsSpawnEvent;

#[derive(Default)]
pub struct WorldLocations {
    positions: HashMap<String, Vec<WorldLocationRect>>,
}

#[derive(Clone, Copy, Debug)]
pub struct WorldLocationRect {
    pub position: Vec2,
    pub size: Vec2,
}

impl WorldLocations {
    pub fn clear(&mut self) {
        self.positions = HashMap::new();
    }

    pub fn add(&mut self, name: &str, position: Vec2, size: Vec2) {
        if !self.positions.contains_key(name) {
            self.positions.insert(String::from(name), vec![]);
        }
        if let Some(vec) = self.positions.get_mut(name) {
            let position_correction =
                position + (size * 0.5 * Vec2::new(1., -1.)) + Vec2::new(-50., 50.);
            vec.push(WorldLocationRect {
                position: position_correction,
                size,
            });
        }
    }

    pub fn get_single_position(&self, name: &str) -> Vec2 {
        if let Some(vec) = self.positions.get(name) {
            vec[0].position
        } else {
            Vec2::ZERO
        }
    }

    pub fn get_single_rect(&self, name: &str) -> WorldLocationRect {
        if let Some(vec) = self.positions.get(name) {
            vec[0]
        } else {
            WorldLocationRect {
                position: Vec2::ZERO,
                size: Vec2::ONE,
            }
        }
    }

    pub fn get_multiple_positions(&self, name: &str) -> Vec<Vec2> {
        if let Some(vec) = self.positions.get(name) {
            vec.iter().map(|i| i.position).collect()
        } else {
            vec![]
        }
    }

    pub fn get_multiple_rect(&self, name: &str) -> Vec<WorldLocationRect> {
        if let Some(vec) = self.positions.get(name) {
            vec.clone()
        } else {
            vec![]
        }
    }
}
