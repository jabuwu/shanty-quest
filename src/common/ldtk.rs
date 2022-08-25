use crate::{common::prelude::*, game::data::town_data::TOWN_NAMES};
use asset_struct::AssetStruct;
use bevy::prelude::*;
use std::collections::HashMap;

use super::grid_combiner::{GridCombiner, GridPoint};

pub struct LdtkPlugin;

impl Plugin for LdtkPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LdtkSpawnEvent>()
            .add_system(ldtk_spawn)
            .add_system(ldtk_load);
    }
}

pub struct LdtkSpawnEvent {
    pub entity: Option<Entity>,
    pub asset: Handle<LdtkAsset>,
    pub position: Vec2,
}

#[derive(Component)]
pub struct Ldtk {
    asset: Handle<LdtkAsset>,
    state: LdtkState,
}

enum LdtkState {
    Unloaded,
    Loaded,
}

impl LdtkState {
    pub fn is_loaded(&self) -> bool {
        if let LdtkState::Loaded = self {
            true
        } else {
            false
        }
    }
}

fn random_color() -> Color {
    Color::rgba(
        rand::random::<f32>(),
        rand::random::<f32>(),
        rand::random::<f32>(),
        0.5,
    )
}

fn ldtk_spawn(mut ev_spawn: EventReader<LdtkSpawnEvent>, mut commands: Commands) {
    for event in ev_spawn.iter() {
        let mut ldtk_entity = if let Some(entity) = event.entity {
            commands.entity(entity)
        } else {
            commands.spawn()
        };
        ldtk_entity
            .insert_bundle(Transform2Bundle {
                transform2: Transform2::from_translation(event.position),
                ..Default::default()
            })
            .insert_bundle(VisibilityBundle::default())
            .insert(Ldtk {
                asset: event.asset.clone(),
                state: LdtkState::Unloaded,
            })
            .insert(Label("Ldtk Map".to_owned()));
    }
}

fn ldtk_load(
    mut query: Query<(Entity, &mut Ldtk)>,
    mut commands: Commands,
    mut texture_atlas_assets: ResMut<Assets<TextureAtlas>>,
    ldtk_assets: Res<Assets<LdtkAsset>>,
    asset_library: Res<AssetLibrary>,
    mut ev_world_locations_spawn: EventWriter<WorldLocationsSpawnEvent>,
    mut world_location: ResMut<WorldLocations>,
    mut map_builder: ResMut<MapBuilder>,
) {
    for (map_entity, mut ldtk) in query.iter_mut() {
        if let Some(ldtk_asset) = ldtk_assets.get(&ldtk.asset) {
            let ldtk_map = &ldtk_asset.map;
            if !ldtk.state.is_loaded() {
                let mut grid_combiner = GridCombiner::new();
                world_location.clear();
                map_builder.reset();
                let mut texture_atlases = HashMap::new();
                for tileset in ldtk_map.defs.tilesets.iter() {
                    let texture_handle = asset_library
                        .from_filename(&format!("levels/{}", tileset.rel_path.as_ref().unwrap()));
                    let texture_atlas = TextureAtlas::from_grid(
                        texture_handle,
                        Vec2::new(tileset.tile_grid_size as f32, tileset.tile_grid_size as f32),
                        (tileset.px_wid / tileset.tile_grid_size) as usize,
                        (tileset.px_hei / tileset.tile_grid_size) as usize,
                    );
                    let texture_atlas_handle = texture_atlas_assets.add(texture_atlas);
                    texture_atlases.insert(tileset.uid as i32, texture_atlas_handle);
                }
                for level in ldtk_map.levels.iter() {
                    let level_entity = commands
                        .spawn_bundle(Transform2Bundle {
                            transform2: Transform2::from_xy(
                                level.world_x as f32,
                                level.world_y as f32 * -1.0,
                            ),
                            ..Default::default()
                        })
                        .insert_bundle(VisibilityBundle::default())
                        .insert(Label(level.identifier.clone()))
                        .id();
                    commands.entity(map_entity).push_children(&[level_entity]);
                    for (idx, layer) in level
                        .layer_instances
                        .as_ref()
                        .unwrap()
                        .iter()
                        .enumerate()
                        .rev()
                    {
                        if layer.identifier == "BG" {
                            continue;
                        }
                        let layer_entity = commands
                            .spawn_bundle(Transform2Bundle {
                                transform2: Transform2::from_xy(0.0, 0.0).with_depth((
                                    DepthLayer::Environment,
                                    0.5 - idx as f32 / 100.0,
                                )),
                                ..Default::default()
                            })
                            .insert_bundle(VisibilityBundle::default())
                            .insert(Label(layer.identifier.clone()))
                            .id();
                        commands.entity(level_entity).push_children(&[layer_entity]);
                        match layer.layer_instance_type.as_str() {
                            "Tiles" => {
                                for tile in layer.grid_tiles.iter() {
                                    let tileset_uid = layer.tileset_def_uid.unwrap_or(-1) as i32;
                                    let tile_entity = ldtk_spawn_tile(
                                        &tile,
                                        tileset_uid,
                                        &texture_atlases,
                                        &mut commands,
                                    );
                                    commands.entity(layer_entity).push_children(&[tile_entity]);
                                }
                            }
                            "AutoLayer" => {
                                for tile in layer.auto_layer_tiles.iter() {
                                    let tileset_uid = layer.tileset_def_uid.unwrap_or(-1) as i32;
                                    let tile_entity = ldtk_spawn_tile(
                                        &tile,
                                        tileset_uid,
                                        &texture_atlases,
                                        &mut commands,
                                    );
                                    commands.entity(layer_entity).push_children(&[tile_entity]);
                                }
                            }
                            "IntGrid" => {
                                if let Some(i) = layer.tileset_def_uid {
                                    for tile in layer.auto_layer_tiles.iter() {
                                        grid_combiner.add_point(GridPoint::new(
                                            (tile.px[0] + level.world_x) / 100,
                                            (tile.px[1] + level.world_y) / -100,
                                        ));
                                        map_builder.add_tile(Vec2::new(
                                            tile.px[0] as f32 + level.world_x as f32,
                                            (tile.px[1] as f32 + level.world_y as f32) * -1.0,
                                        ));
                                        let tile_entity = ldtk_spawn_tile(
                                            &tile,
                                            i as i32,
                                            &texture_atlases,
                                            &mut commands,
                                        );
                                        commands.entity(layer_entity).push_children(&[tile_entity]);
                                    }
                                }
                            }
                            "Entities" => {
                                for entity in layer.entity_instances.iter() {
                                    for name in TOWN_NAMES.iter() {
                                        let town_name = entity.identifier.replace("_", " ");
                                        if town_name == *name {
                                            map_builder.add_label(
                                                Vec2::new(
                                                    entity.px[0] as f32 + level.world_x as f32,
                                                    (entity.px[1] as f32 + level.world_y as f32)
                                                        * -1.0,
                                                ),
                                                name,
                                            );
                                        }
                                    }
                                    world_location.add(
                                        &entity.identifier,
                                        Vec2::new(
                                            entity.px[0] as f32 + level.world_x as f32,
                                            (entity.px[1] as f32 + level.world_y as f32) * -1.,
                                        ),
                                        Vec2::new(entity.width as f32, entity.height as f32),
                                    );
                                }
                            }
                            _ => (),
                        }
                    }
                }
                let rects = grid_combiner.combine();
                for rect in rects.iter() {
                    let (mut pos, mut size) = rect.to_position_size();
                    pos *= 100.;
                    size *= 100.;
                    commands
                        .spawn_bundle(SpriteBundle {
                            sprite: Sprite {
                                color: random_color(),
                                custom_size: size.into(),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(
                            Transform2::from_translation(pos).with_depth((DepthLayer::Front, 1.)),
                        )
                        .insert(Collision {
                            shape: CollisionShape::Rect { size },
                            flags: COLLISION_FLAG,
                        });
                }
                ev_world_locations_spawn.send_default();
                ldtk.state = LdtkState::Loaded;
            }
        }
    }
}

fn ldtk_spawn_tile(
    tile: &ldtk2::TileInstance,
    tileset_uid: i32,
    texture_atlases: &HashMap<i32, Handle<TextureAtlas>>,
    commands: &mut Commands,
) -> Entity {
    let mut sprite = TextureAtlasSprite::new(tile.t as usize);
    match tile.f {
        1 => sprite.flip_x = true,
        2 => sprite.flip_y = true,
        3 => {
            sprite.flip_x = true;
            sprite.flip_y = true
        }
        _ => (),
    }
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite,
            texture_atlas: texture_atlases[&tileset_uid].clone(),
            ..Default::default()
        })
        .insert(Transform2::from_xy(
            tile.px[0] as f32,
            tile.px[1] as f32 * -1.0,
        ))
        .id()
}
