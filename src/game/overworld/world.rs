use crate::game::prelude::*;
use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WorldLoadEvent>().add_system(world_spawn);
    }
}

#[derive(Default, Clone, Copy)]
pub struct WorldLoadEvent;

#[derive(Component)]
pub struct World;

fn world_spawn(
    mut ev_spawn: EventReader<WorldLoadEvent>,
    mut commands: Commands,
    mut ev_spawn_island: EventWriter<IslandSpawnEvent>,
) {
    for _ in ev_spawn.iter() {
        let island_entity = commands.spawn().id();
        ev_spawn_island.send(IslandSpawnEvent {
            entity: island_entity,
            town: TownData {
                name: "Tortuga".to_string(),
                position: Vec2::new(-300., -200.),
            },
        });
        let island_entity = commands.spawn().id();
        ev_spawn_island.send(IslandSpawnEvent {
            entity: island_entity,
            town: TownData {
                name: "Raven Rock".to_string(),
                position: Vec2::new(200., 50.),
            },
        });
    }
}