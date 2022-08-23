use crate::common::prelude::*;
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
    mut ev_ldtk_spawn: EventWriter<LdtkSpawnEvent>,
    mut ev_ocean_spawn: EventWriter<OceanSpawnEvent>,
    mut ev_spawn_island: EventWriter<TownSpawnEvent>,
    asset_library: Res<AssetLibrary>,
) {
    for _ in ev_spawn.iter() {
        ev_ocean_spawn.send_default();
        ev_ldtk_spawn.send(LdtkSpawnEvent {
            entity: None,
            asset: asset_library.level_test.clone(),
            position: Vec2::new(-800., 350.),
        });
        let island_entity = commands.spawn().id();
        ev_spawn_island.send(TownSpawnEvent {
            entity: island_entity,
            town: TownData {
                name: "Tortuga".to_string(),
                position: Vec2::new(887., -353.),
                spawn_offset: Vec2::new(0., -300.),
            },
        });
        let island_entity = commands.spawn().id();
        ev_spawn_island.send(TownSpawnEvent {
            entity: island_entity,
            town: TownData {
                name: "Raven Rock".to_string(),
                position: Vec2::new(-30., 247.),
                spawn_offset: Vec2::new(0., -300.),
            },
        });
    }
}
