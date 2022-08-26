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
        app.add_event::<KnockbackEvent>()
            .add_system(
                character_controller_update
                    .label(CharacterControllerSystems::Update)
                    .before(OverworldCameraSystems::Update),
            )
            .add_system(character_controller_knockback);
    }
}

#[derive(Component, Default)]
pub struct CharacterController {
    pub movement: Vec2,
    pub speed: f32,
    pub force_facing: Option<Facing>,
    pub knockback: Vec2,
    pub knockback_resistance: f32,
}

#[derive(Clone, Copy)]
pub struct KnockbackEvent {
    pub entity: Entity,
    pub force: Vec2,
}

#[derive(Component)]
pub struct CharacterControllerDestination {
    pub target: Vec2,
}

fn character_controller_update(
    query: Query<Entity, (With<CharacterController>, With<Transform2>, With<Collision>)>,
    mut queries: ParamSet<(
        Query<(
            Entity,
            &mut CharacterController,
            &mut Transform2,
            &Collision,
            &GlobalTransform,
            Option<&Dash>,
            Option<&CharacterControllerDestination>,
        )>,
        Query<(Entity, &GlobalTransform, &Collision)>,
    )>,
    mut collision_query: ResMut<CollisionQuery>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for entity in query.iter() {
        collision_query.update(&queries.p1());
        if let Ok((
            entity,
            mut character_controller,
            mut transform,
            collision,
            global_transform,
            dash,
            destination,
        )) = queries.p0().get_mut(entity)
        {
            let mut velocity = character_controller.movement;
            character_controller.force_facing = None;
            if let Some(destination) = destination {
                velocity = (destination.target - global_transform.translation().truncate()) / 200.;
                if let Some(facing) = Facing::from_vec(velocity) {
                    character_controller.force_facing = Some(facing);
                }
                if velocity.length() < 0.1 {
                    commands
                        .entity(entity)
                        .remove::<CharacterControllerDestination>();
                }
            }
            if velocity.length_squared() > 1. {
                velocity = velocity.normalize();
            }
            velocity *= character_controller.speed * time.delta_seconds();
            if let Some(dash) = dash {
                velocity += dash.velocity * time.delta_seconds();
            }
            velocity += character_controller.knockback;
            character_controller.knockback *= 0.000001_f32.powf(time.delta_seconds());
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

fn character_controller_knockback(
    mut query: Query<&mut CharacterController>,
    mut ev_knockback: EventReader<KnockbackEvent>,
) {
    for event in ev_knockback.iter() {
        if let Ok(mut character_controller) = query.get_mut(event.entity) {
            let resistance = 1. - character_controller.knockback_resistance;
            character_controller.knockback += event.force * resistance;
        }
    }
}
