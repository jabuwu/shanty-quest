use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;

pub struct IslandPlugin;

impl Plugin for IslandPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<IslandSpawnEvent>().add_system(island_spawn);
    }
}

pub struct IslandSpawnEvent {
    pub entity: Entity,
    pub town: TownData,
}

#[derive(Component)]
pub struct Island {
    pub town: TownData,
}

fn island_spawn(mut ev_spawn: EventReader<IslandSpawnEvent>, mut commands: Commands) {
    for event in ev_spawn.iter() {
        commands
            .entity(event.entity)
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Vec2::new(32., 128.).into(),
                    color: Color::BEIGE,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(
                Transform2::from_translation(event.town.position)
                    .with_depth((DepthLayer::Entity, 0.)),
            )
            .insert(Island {
                town: event.town.clone(),
            })
            .insert(YDepth::default());
    }
}
