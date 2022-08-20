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
                    custom_size: Vec2::new(32., 32.).into(),
                    color: Color::SEA_GREEN,
                    ..Default::default()
                },
                transform: Transform::from_translation(event.town.position.extend(0.1)),
                ..Default::default()
            })
            .insert(Island {
                town: event.town.clone(),
            });
    }
}
