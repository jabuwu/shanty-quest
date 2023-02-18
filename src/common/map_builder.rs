use bevy::prelude::*;

pub struct MapBuilderPlugin;

impl Plugin for MapBuilderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MapBuilder>();
    }
}

#[derive(Default, Resource)]
pub struct MapBuilder {
    pub tiles: Vec<Vec2>,
    pub labels: Vec<(Vec2, String)>,
}

impl MapBuilder {
    pub fn offset(&self) -> Vec2 {
        Vec2::new(100., 200.)
    }

    pub fn size(&self) -> Vec2 {
        Vec2::new(12000., 12000.)
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn add_tile(&mut self, pos: Vec2) {
        let mut new_pos = pos;
        new_pos -= self.offset();
        new_pos.y *= -1.;
        if new_pos.x >= 0.
            && new_pos.y >= 0.
            && new_pos.x <= self.size().x
            && new_pos.y <= self.size().y
        {
            new_pos.y *= -1.;
            self.tiles.push(new_pos);
        }
    }

    pub fn add_label(&mut self, pos: Vec2, label: &str) {
        let mut new_pos = pos;
        new_pos -= self.offset();
        new_pos.y *= -1.;
        if new_pos.x >= 0.
            && new_pos.y >= 0.
            && new_pos.x <= self.size().x
            && new_pos.y <= self.size().y
        {
            new_pos.y *= -1.;
            self.labels.push((new_pos, String::from(label)));
        }
    }

    pub fn world_to_map(&self, position: Vec2) -> Vec2 {
        let mut map_position = position;
        map_position -= self.offset();
        map_position /= self.size();
        map_position += Vec2::new(-0.5, 0.5);
        map_position
    }
}
