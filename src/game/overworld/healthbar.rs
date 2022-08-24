use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;

const HEALTHBAR_BORDER: Vec2 = Vec2::new(8., 8.);

pub struct HealthbarPlugin;

impl Plugin for HealthbarPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<HealthbarSpawnEvent>()
            .add_system(healthbar_spawn)
            .add_system(healthbar_update);
    }
}

#[derive(Default, Clone, Copy)]
pub struct HealthbarSpawnEvent {
    pub entity: Option<Entity>,
    pub offset: Vec2,
    pub size: Vec2,
}

#[derive(Component)]
pub struct Healthbar {
    size: Vec2,
}

#[derive(Component)]
struct HealthbarValue {
    offset: Vec2,
}

fn healthbar_spawn(mut ev_spawn: EventReader<HealthbarSpawnEvent>, mut commands: Commands) {
    for event in ev_spawn.iter() {
        let mut entity = if let Some(entity) = event.entity {
            commands.entity(entity)
        } else {
            commands.spawn()
        };
        entity
            .insert(Healthbar { size: event.size })
            .with_children(|parent| {
                parent
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(event.size + HEALTHBAR_BORDER),
                            color: Color::BLACK,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(
                        Transform2::from_translation(event.offset)
                            .with_depth(DEPTH_LAYER_HEALTHBAR_BORDER),
                    );
                parent
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(event.size),
                            color: Color::RED,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(
                        Transform2::from_translation(event.offset)
                            .with_depth(DEPTH_LAYER_HEALTHBAR),
                    )
                    .insert(HealthbarValue {
                        offset: event.offset,
                    });
            });
    }
}

fn healthbar_update(
    healthbar_query: Query<(&Healthbar, &Health, &Children)>,
    mut healthbar_value_query: Query<(&mut Transform2, &HealthbarValue)>,
) {
    for (healthbar, health, children) in healthbar_query.iter() {
        for child in children.iter() {
            if let Ok((mut healthbar_value_transform, healthbar_value)) =
                healthbar_value_query.get_mut(*child)
            {
                let amount = health.value / health.max;
                healthbar_value_transform.translation =
                    healthbar_value.offset - Vec2::new(healthbar.size.x * 0.5 * (1. - amount), 0.);
                healthbar_value_transform.scale = Vec2::new(amount, 1.);
            }
        }
    }
}
