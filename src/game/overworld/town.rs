use crate::common::prelude::*;
use crate::game::data::town_data::TOWN_NAMES;
use crate::game::prelude::*;
use bevy::prelude::*;

pub struct TownPlugin;

impl Plugin for TownPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TownSpawnEvent>()
            .add_system(town_spawn)
            .add_system(town_world_spawn);
    }
}

pub struct TownSpawnEvent {
    pub entity: Option<Entity>,
    pub town: TownData,
    pub position: Vec2,
}

#[derive(Component)]
pub struct Town {
    pub town: TownData,
}

fn town_spawn(
    mut ev_spawn: EventReader<TownSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for event in ev_spawn.iter() {
        let mut entity = if let Some(entity) = event.entity {
            commands.entity(entity)
        } else {
            commands.spawn()
        };
        entity
            .insert_bundle(SpriteBundle {
                texture: asset_library.sprite_overworld_city.clone(),
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
            .insert(Label(format!("Town: {}", event.town.name)))
            .with_children(|parent| {
                parent
                    .spawn_bundle(Text2dBundle {
                        text: Text::from_section(
                            event.town.name.clone(),
                            TextStyle {
                                font: asset_library.font_default.clone(),
                                font_size: 48.0,
                                color: Color::BLACK,
                            },
                        )
                        .with_alignment(TextAlignment {
                            horizontal: HorizontalAlign::Center,
                            vertical: VerticalAlign::Center,
                        }),
                        ..Default::default()
                    })
                    .insert(Transform2::from_xy(0., 130.).with_depth(DEPTH_LAYER_TOWN_NAME));
            });
    }
}

fn town_world_spawn(
    mut ev_spawn: EventReader<WorldLocationsSpawnEvent>,
    world_locations: Res<WorldLocations>,
    mut ev_rubble_spawn: EventWriter<TownSpawnEvent>,
) {
    for _ in ev_spawn.iter() {
        for name in TOWN_NAMES.iter() {
            let positions = world_locations.get_multiple_positions(*name);
            for position in positions {
                ev_rubble_spawn.send(TownSpawnEvent {
                    position,
                    entity: None,
                    town: TownData {
                        name: String::from(*name),
                        position: position,
                        spawn_offset: Vec2::new(0., -300.),
                    },
                });
            }
        }
    }
}
