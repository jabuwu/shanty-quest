use bevy::prelude::*;

pub struct TimeToLivePlugin;

impl Plugin for TimeToLivePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(ttl_update);
    }
}

#[derive(Component)]
pub struct TimeToLive(pub f32);

fn ttl_update(
    mut query: Query<(Entity, &mut TimeToLive)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, mut ttl) in query.iter_mut() {
        ttl.0 -= time.delta_seconds();
        if ttl.0 <= 0. {
            commands.entity(entity).despawn();
        }
    }
}
