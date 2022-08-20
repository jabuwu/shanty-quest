use bevy::prelude::*;

pub struct CannonBallPlugin;

impl Plugin for CannonBallPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CannonBallSpawnEvent>()
            .add_system(cannon_ball_spawn)
            .add_system(cannon_ball_move);
    }
}

#[derive(Default, Clone, Copy)]
pub struct CannonBallSpawnEvent {
    pub entity: Option<Entity>,
    pub position: Vec2,
    pub velocity: Vec2,
}

#[derive(Component)]
pub struct CannonBall {
    pub velocity: Vec2,
}

fn cannon_ball_spawn(mut ev_spawn: EventReader<CannonBallSpawnEvent>, mut commands: Commands) {
    for event in ev_spawn.iter() {
        let mut entity = if let Some(entity) = event.entity {
            commands.entity(entity)
        } else {
            commands.spawn()
        };
        entity
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Vec2::new(8., 8.).into(),
                    color: Color::BLACK,
                    ..Default::default()
                },
                transform: Transform::from_translation(event.position.extend(0.4)),
                ..Default::default()
            })
            .insert(CannonBall {
                velocity: event.velocity,
            });
    }
}

fn cannon_ball_move(mut query: Query<(&mut Transform, &CannonBall)>, time: Res<Time>) {
    for (mut transform, cannon_ball) in query.iter_mut() {
        transform.translation += (cannon_ball.velocity * time.delta_seconds()).extend(0.);
    }
}
