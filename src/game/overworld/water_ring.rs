use crate::common::prelude::*;
use bevy::prelude::*;

pub struct WaterRingPlugin;

const MAX_LIFE_TIME: f32 = 0.75;

impl Plugin for WaterRingPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WaterRingSpawnEvent>()
            .add_system(water_ring_spawn)
            .add_system(water_ring_update);
    }
}

#[derive(Default, Clone, Copy)]
pub struct WaterRingSpawnEvent {
    pub entity: Option<Entity>,
    pub position: Vec2,
}

#[derive(Component)]
pub struct WaterRing {
    pub life_time: f32
}

fn water_ring_spawn(
    mut ev_spawn: EventReader<WaterRingSpawnEvent>, 
    mut commands: Commands,
    asset_library: Res<AssetLibrary>
) {
    for event in ev_spawn.iter() {
        let mut entity = if let Some(entity) = event.entity {
            commands.entity(entity)
        } else {
            commands.spawn()
        };

        entity
            .insert_bundle(SpriteBundle {
                texture: asset_library.sprite_water_ring_vfx.clone(),
                ..Default::default()
            })
            .insert(Transform2::from_translation(event.position)
                .with_scale(Vec2::ZERO)
                .with_depth((DepthLayer::Environment, 0.015))
            )
            .insert(WaterRing {
                life_time: MAX_LIFE_TIME
            });
    }
}

fn water_ring_update(
    mut query: Query<(&mut Transform2, &mut WaterRing, Entity, &mut Sprite)>, 
    mut commands: Commands,
    time: Res<Time>
) {
    for (mut transform, mut water_ring, entity, mut sprite) in query.iter_mut() {
        water_ring.life_time -= time.delta_seconds(); 
        if water_ring.life_time <= 0.0 {
            commands.entity(entity).despawn();
            return;
        }
        let interp = water_ring.life_time / MAX_LIFE_TIME;
        sprite.color.set_a(interp);
        transform.scale = Vec2::ONE * (1.0 - interp);
    }
}
