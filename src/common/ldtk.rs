use crate::common::prelude::*;
use asset_struct::AssetStruct;
use bevy::prelude::*;
use std::collections::HashMap;

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
) {
    for (map_entity, mut ldtk) in query.iter_mut() {
        if let Some(ldtk_asset) = ldtk_assets.get(&ldtk.asset) {
            let ldtk_map = &ldtk_asset.map;
            if !ldtk.state.is_loaded() {
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
                            _ => (),
                        }
                    }
                }
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