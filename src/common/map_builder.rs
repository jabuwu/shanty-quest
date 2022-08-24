use bevy::prelude::*;

pub struct MapBuilderPlugin;

impl Plugin for MapBuilderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MapBuilder>();
    }
}

#[derive(Default)]
pub struct MapBuilder {
    pub tiles: Vec<Vec2>,
}

impl MapBuilder {
    pub fn offset(&self) -> Vec2 {
        Vec2::new(100., -100.)
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
}
