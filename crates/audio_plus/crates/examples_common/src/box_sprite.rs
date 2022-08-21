use bevy::prelude::*;

pub fn box_sprite(position: Vec2, color: Color) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            custom_size: Vec2::new(32., 32.).into(),
            color,
            ..Default::default()
        },
        transform: Transform::from_translation(position.extend(0.)),
        ..Default::default()
    }
}
