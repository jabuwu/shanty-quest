use crate::common::prelude::*;
use bevy::prelude::*;

pub struct OceanPlugin;

impl Plugin for OceanPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OceanSpawnEvent>().add_system(ocean_spawn);
    }
}

#[derive(Default, Clone, Copy)]
pub struct OceanSpawnEvent;

#[derive(Component)]
pub struct Ocean;

const WATER_WIDTH: f32 = 150.;
const WATER_HEIGHT: f32 = 150.;
fn ocean_spawn(
    mut ev_spawn: EventReader<OceanSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for _ in ev_spawn.iter() {
        for x in 0..30 {
            for y in 0..30 {
                commands
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            custom_size: Vec2::new(WATER_WIDTH, WATER_HEIGHT).into(),
                            ..Default::default()
                        },
                        texture: asset_library.level_test_water_tilemap.clone(),
                        ..Default::default()
                    })
                    .insert(Transform2::from_xy(
                        x as f32 * WATER_WIDTH - 1000.,
                        y as f32 * WATER_HEIGHT - 1000.,
                    ))
                    .insert(Ocean);
            }
        }
    }
}
