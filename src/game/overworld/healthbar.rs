use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;

const HEALTHBAR_SIZE: Vec2 = Vec2::new(256., 24.);
const HEALTHBAR_BORDER: Vec2 = Vec2::new(8., 8.);

pub struct HealthbarPlugin;

impl Plugin for HealthbarPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<HealthbarSpawnEvent>()
            .add_system(healthbar_spawn)
            .add_system(healthbar_update)
            .add_system(healthbar_test);
    }
}

#[derive(Default, Clone, Copy)]
pub struct HealthbarSpawnEvent {
    pub entity: Option<Entity>,
    pub offset: Vec2,
}

#[derive(Component)]
pub struct Healthbar;

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
        entity.insert(Healthbar).with_children(|parent| {
            parent
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(HEALTHBAR_SIZE + HEALTHBAR_BORDER),
                        color: Color::BLACK,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(
                    Transform2::from_translation(event.offset)
                        .with_depth((DepthLayer::Front, 0.11)),
                );
            parent
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(HEALTHBAR_SIZE),
                        color: Color::RED,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(
                    Transform2::from_translation(event.offset)
                        .with_depth((DepthLayer::Front, 0.111)),
                )
                .insert(HealthbarValue {
                    offset: event.offset,
                });
        });
    }
}

fn healthbar_update(
    healthbar_query: Query<(&Health, &Children), With<Healthbar>>,
    mut healthbar_value_query: Query<(&mut Transform2, &HealthbarValue)>,
) {
    for (health, children) in healthbar_query.iter() {
        for child in children.iter() {
            if let Ok((mut healthbar_value_transform, healthbar_value)) =
                healthbar_value_query.get_mut(*child)
            {
                let amount = health.value / health.max;
                healthbar_value_transform.translation =
                    healthbar_value.offset - Vec2::new(HEALTHBAR_SIZE.x * 0.5 * (1. - amount), 0.);
                healthbar_value_transform.scale = Vec2::new(amount, 1.);
            }
        }
    }
}

fn healthbar_test(mut query: Query<&mut Health>, time: Res<Time>) {
    for mut health in query.iter_mut() {
        health.value -= time.delta_seconds() * 10.;
        if health.value < 0. {
            health.value += health.max;
        }
    }
}
