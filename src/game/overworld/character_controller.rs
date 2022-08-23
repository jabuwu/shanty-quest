use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum CharacterControllerSystems {
    Update,
}

pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            character_controller_update
                .label(CharacterControllerSystems::Update)
                .before(OverworldCameraSystems::Update),
        );
    }
}

#[derive(Component)]
pub struct CharacterController {
    pub movement: Vec2,
    pub speed: f32,
}

fn character_controller_update(
    query: Query<Entity, (With<CharacterController>, With<Transform2>, With<Collision>)>,
    mut queries: ParamSet<(
        Query<(
            Entity,
            &CharacterController,
            &mut Transform2,
            &Collision,
            Option<&Dash>,
        )>,
        Query<(Entity, &GlobalTransform, &Collision)>,
    )>,
    mut collision_query: ResMut<CollisionQuery>,
    time: Res<Time>,
) {
    for entity in query.iter() {
        collision_query.update(&queries.p1());
        if let Ok((entity, character_controller, mut transform, collision, dash)) =
            queries.p0().get_mut(entity)
        {
            let mut velocity = character_controller.movement;
            if velocity.length_squared() > 1. {
                velocity = velocity.normalize();
            }
            velocity *= character_controller.speed * time.delta_seconds();
            if let Some(dash) = dash {
                velocity += dash.velocity * time.delta_seconds();
            }
            let velocity_x = Vec2::X * velocity;
            let velocity_y = Vec2::Y * velocity;
            let collision_filters = CollisionFilter {
                exclude_entity: entity,
                flags: 1,
            };
            if collision_query
                .check_moving(
                    transform.translation,
                    velocity_x * 3.,
                    collision.shape,
                    Some(collision_filters),
                )
                .is_none()
            {
                transform.translation += velocity_x;
            }
            if collision_query
                .check_moving(
                    transform.translation,
                    velocity_y * 3.,
                    collision.shape,
                    Some(collision_filters),
                )
                .is_none()
            {
                transform.translation += velocity_y;
            }
        }
    }
}
