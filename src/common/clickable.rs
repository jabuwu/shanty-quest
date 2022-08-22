use crate::common::prelude::*;
use bevy::prelude::*;

pub struct ClickablePlugin;

impl Plugin for ClickablePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(clickable_update);
    }
}

#[derive(Component, Default)]
pub struct Clickable {
    pub shape: CollisionShape,

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
}

fn clickable_update(
    mut query: Query<(&mut Clickable, &Transform2)>,
    mouse: Res<Mouse>,
    input: Res<Input<MouseButton>>,
) {
    for (mut clickable, transform) in query.iter_mut() {
        clickable.last_hovered = clickable.hovered;
        clickable.last_clicked = clickable.clicked;
        clickable.confirmed = false;
        clickable.hovered =
            clickable
                .shape
                .overlaps(transform.translation, CollisionShape::Point, mouse.position);
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
