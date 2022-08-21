use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;

pub struct TownPlugin;

impl Plugin for TownPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TownSpawnEvent>().add_system(town_spawn);
    }
}

pub struct TownSpawnEvent {
    pub entity: Entity,
    pub town: TownData,
}

#[derive(Component)]
pub struct Town {
    pub town: TownData,
}

fn town_spawn(mut ev_spawn: EventReader<TownSpawnEvent>, mut commands: Commands) {
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
            .insert(Town {
                town: event.town.clone(),
            })
            .insert(YDepth::default())
            .insert(Label(format!("Town: {}", event.town.name)));
    }
}
