use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;

pub struct RubblePlugin;

impl Plugin for RubblePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RubbleSpawnEvent>()
            .add_system(rubble_spawn.before(HealthbarSystems::Spawn))
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
    mut ev_healthbar_spawn: EventWriter<HealthbarSpawnEvent>,
) {
    for event in ev_spawn.iter() {
        let entity = commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Vec2::new(128., 128.).into(),
                    color: Color::DARK_GRAY,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(
                Transform2::from_translation(event.position).with_depth((DepthLayer::Entity, 0.)),
            )
            .insert(Rubble)
            .insert(Label("Rubble".to_owned()))
            .insert(YDepth::default())
            .insert(Health::new(3.))
            .insert(Hitbox {
                shape: CollisionShape::Rect {
                    size: Vec2::new(128., 128.),
                },
                for_entity: None,
                flags: DAMAGE_FLAG_ENVIRONMENT,
            })
            .insert(Collision {
                shape: CollisionShape::Rect {
                    size: Vec2::new(128., 128.),
                },
                flags: COLLISION_FLAG,
            })
            .insert(AutoDamage {
                despawn: true,
                ..Default::default()
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
