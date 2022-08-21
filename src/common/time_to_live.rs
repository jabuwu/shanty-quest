use bevy::prelude::*;

pub struct TimeToLivePlugin;

impl Plugin for TimeToLivePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(ttl_update);
    }
}

#[derive(Component)]
pub struct TimeToLive {
    pub seconds: f32,
}

impl TimeToLive {
    pub fn new(seconds: f32) -> Self {
        Self { seconds }
    }
}

fn ttl_update(
    mut query: Query<(Entity, &mut TimeToLive)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, mut ttl) in query.iter_mut() {
        ttl.seconds -= time.delta_seconds();
        if ttl.seconds <= 0. {
            commands.entity(entity).despawn();
        }
    }
}
