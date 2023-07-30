use crate::common::prelude::*;
use bevy::prelude::*;

pub struct ClickablePlugin;

impl Plugin for ClickablePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, clickable_update);
    }
}

#[derive(Component, Default)]
pub struct Clickable {
    pub shape: CollisionShape,
    pub use_global: bool,
    pub offset: Vec2,

    pub disabled: bool,

    pub hovered: bool,
    pub last_hovered: bool,

    pub clicked: bool,
    pub last_clicked: bool,

    pub confirmed: bool,
}

impl Clickable {
    pub fn new(shape: CollisionShape) -> Self {
        Self {
            shape,
            ..Default::default()
        }
    }

    pub fn just_hovered(&self) -> bool {
        return self.hovered && !self.last_hovered;
    }

    pub fn just_clicked(&self) -> bool {
        return self.clicked && !self.last_clicked;
    }

    pub fn just_released(&self) -> bool {
        return !self.clicked && self.last_clicked;
    }
}

fn clickable_update(
    mut query: Query<(&mut Clickable, &Transform2, &GlobalTransform)>,
    mouse: Res<Mouse>,
    input: Res<Input<MouseButton>>,
) {
    for (mut clickable, transform, global_transform) in query.iter_mut() {
        let (mut translation, shape) = if clickable.use_global {
            if let CollisionShape::Rect { size } = &clickable.shape {
                let (scale, _, _) = global_transform.to_scale_rotation_translation();
                (
                    global_transform.translation().truncate(),
                    CollisionShape::Rect {
                        size: *size * scale.truncate(),
                    },
                )
            } else {
                (global_transform.translation().truncate(), clickable.shape)
            }
        } else {
            (transform.translation, clickable.shape)
        };
        translation += clickable.offset;
        clickable.last_hovered = clickable.hovered;
        clickable.last_clicked = clickable.clicked;
        clickable.confirmed = false;
        clickable.hovered = shape.overlaps(translation, CollisionShape::Point, mouse.position);
        if clickable.hovered && input.just_pressed(MouseButton::Left) {
            clickable.clicked = true;
        }
        if clickable.clicked && input.just_released(MouseButton::Left) {
            clickable.clicked = false;
            if clickable.hovered {
                clickable.confirmed = true;
            }
        }
    }
}
