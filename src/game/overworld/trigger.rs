use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;

pub struct TriggerPlugin;

impl Plugin for TriggerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(trigger_check);
    }
}

#[derive(Component)]
pub struct Trigger {
    shape: CollisionShape,
    inside: bool,
}

impl Trigger {
    pub fn new(shape: CollisionShape) -> Self {
        Self {
            shape,
            inside: false,
        }
    }

    pub fn triggered(&self) -> bool {
        self.inside
    }
}

fn trigger_check(
    mut trigger_query: Query<(Entity, &mut Trigger)>,
    player_query: Query<(Entity, &Collision), With<Player>>,
    transform_query: Query<&Transform2>,
) {
    for (trigger_entity, mut trigger) in trigger_query.iter_mut() {
        let trigger_translation = if let Ok(transform) = transform_query.get(trigger_entity) {
            transform.translation
        } else {
            continue;
        };
        for (player_entity, player_collision) in player_query.iter() {
            let player_translation = if let Ok(transform) = transform_query.get(player_entity) {
                transform.translation
            } else {
                continue;
            };
            trigger.inside = player_collision.shape.overlaps(
                player_translation,
                trigger.shape,
                trigger_translation,
            );
        }
    }
}
