use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;

pub struct RubblePlugin;

impl Plugin for RubblePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RubbleSpawnEvent>()
            .add_system(rubble_spawn.before(HealthbarSystem::Spawn))
            .add_system(rubble_world_spawn);
    }
}

#[derive(Default, Clone, Copy)]
pub struct RubbleSpawnEvent {
    pub position: Vec2,
}

#[derive(Component)]
pub struct Rubble;

fn rubble_spawn(
    mut ev_spawn: EventReader<RubbleSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
    mut ev_healthbar_spawn: EventWriter<HealthbarSpawnEvent>,
) {
    for event in ev_spawn.iter() {
        let entity = commands
            .spawn((
                TransformBundle::default(),
                VisibilityBundle::default(),
                Transform2::from_translation(event.position),
                Rubble,
                Health::new(1.),
                Hitbox {
                    shape: CollisionShape::Rect {
                        size: Vec2::new(128., 128.),
                    },
                    for_entity: None,
                    flags: DAMAGE_FLAG_ENVIRONMENT,
                },
                Collision {
                    shape: CollisionShape::Rect {
                        size: Vec2::new(128., 128.),
                    },
                    flags: COLLISION_FLAG,
                },
                AutoDamage {
                    despawn: true,
                    ..Default::default()
                },
            ))
            .with_children(|parent| {
                parent.spawn((
                    SpriteBundle {
                        texture: asset_library.sprite_rubble.clone(),
                        ..Default::default()
                    },
                    Transform2::new()
                        .with_scale(Vec2::new(0.25, 0.25))
                        .with_depth((DepthLayer::Entity, 0.)),
                    YDepth::default(),
                ));
            })
            .id();
        ev_healthbar_spawn.send(HealthbarSpawnEvent {
            entity: Some(entity),
            offset: Vec2::new(0., 75.),
            size: Vec2::new(80., 6.),
        });
    }
}

fn rubble_world_spawn(
    mut ev_spawn: EventReader<WorldLocationsSpawnEvent>,
    world_locations: Res<WorldLocations>,
    mut ev_rubble_spawn: EventWriter<RubbleSpawnEvent>,
) {
    for _ in ev_spawn.iter() {
        let positions = world_locations.get_multiple_positions("Rubble");
        for position in positions {
            ev_rubble_spawn.send(RubbleSpawnEvent { position });
        }
    }
}
