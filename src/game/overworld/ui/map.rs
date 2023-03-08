use crate::common::prelude::*;
use crate::game::prelude::*;
use audio_plus::prelude::*;
use bevy::prelude::*;
use bevy::sprite::Anchor;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(map_update_player)
            .add_system(map_update_objective)
            .add_cutscene::<MapCutscene>()
            .add_system(map_input.in_set(OnUpdate(AppState::Overworld)));
    }
}

#[derive(Default, Debug, Clone, Resource)]
pub struct MapCutscene;

impl Cutscene for MapCutscene {
    fn build(cutscene: &mut CutsceneBuilder) {
        cutscene.add_step(map_open, map_wait_for_close);
        cutscene.add_quick_step(map_close);
    }
}

fn map_open(
    mut commands: Commands,
    map_builder: Res<MapBuilder>,
    asset_library: Res<AssetLibrary>,
) {
    commands
        .spawn(VisibilityBundle::default())
        .insert(TransformBundle::default())
        .insert(Transform2::from_xy(0., 0.).without_pixel_perfect())
        .insert(FollowCamera { offset: Vec2::ZERO })
        .insert(Map)
        .insert(
            AudioPlusSource::new(asset_library.sound_effects.sfx_overworld_map_open.clone())
                .as_playing(),
        )
        .with_children(|parent| {
            parent
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::BLACK,
                        ..Default::default()
                    },
                    texture: asset_library.sprite_map_compass.clone(),
                    ..Default::default()
                })
                .insert(
                    Transform2::from_xy(230., 230.)
                        .with_scale(Vec2::ONE * 0.25)
                        .with_depth(DEPTH_LAYER_MAP_COMPASS),
                );
            parent
                .spawn(SpriteBundle {
                    texture: asset_library.sprite_map_bg.clone(),
                    ..Default::default()
                })
                .insert(Transform2::new().with_depth(DEPTH_LAYER_MAP_BACK))
                .with_children(|parent| {
                    parent
                        .spawn(SpriteBundle {
                            sprite: Sprite {
                                custom_size: Vec2::new(1., 1.).into(),
                                color: Color::rgba_u8(255, 217, 162, 0),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(
                            Transform2::from_xy(0., 0.)
                                .with_scale(Vec2::new(620., 620.))
                                .with_depth(DEPTH_LAYER_MAP_BACK_COLOR)
                                .without_pixel_perfect(),
                        )
                        .with_children(|parent| {
                            for tile in map_builder.tiles.iter() {
                                let mut alpha: f32 = 1.;
                                let pos = *tile / (map_builder.size()) + Vec2::new(-0.5, 0.5);
                                alpha *= ((0.5 - pos.x) * 60.).abs().clamp(0., 1.);
                                alpha *= ((-0.5 - pos.x) * 60.).abs().clamp(0., 1.);
                                alpha *= ((0.5 - pos.y) * 60.).abs().clamp(0., 1.);
                                alpha *= ((-0.5 - pos.y) * 60.).abs().clamp(0., 1.);
                                parent
                                    .spawn(SpriteBundle {
                                        sprite: Sprite {
                                            custom_size: Vec2::new(
                                                102. / map_builder.size().x,
                                                102. / map_builder.size().y,
                                            )
                                            .into(),
                                            color: Color::rgba_u8(
                                                175,
                                                95,
                                                50,
                                                (255. * alpha) as u8,
                                            ),
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    })
                                    .insert(
                                        Transform2::from_translation(pos)
                                            .with_depth(DEPTH_LAYER_MAP_TILE),
                                    );
                            }
                            for label in map_builder.labels.iter() {
                                parent
                                    .spawn(SpriteBundle {
                                        texture: asset_library.sprite_map_icon_town.clone(),
                                        ..Default::default()
                                    })
                                    .insert(
                                        Transform2::from_translation(
                                            (label.0 + Vec2::new(0., 50.)) / (map_builder.size())
                                                + Vec2::new(-0.5, 0.5),
                                        )
                                        .with_scale(Vec2::ONE * (5.5 / map_builder.size().x))
                                        .with_depth(DEPTH_LAYER_MAP_LABEL),
                                    );
                                parent
                                    .spawn(Text2dBundle {
                                        text: Text::from_section(
                                            label.1.clone(),
                                            TextStyle {
                                                font: asset_library.font_bold.clone(),
                                                font_size: 66.,
                                                color: Color::BLACK,
                                            },
                                        )
                                        .with_alignment(TextAlignment::Center),
                                        text_anchor: Anchor::Center,
                                        ..Default::default()
                                    })
                                    .insert(
                                        Transform2::from_translation(
                                            (label.0 + Vec2::new(0., 550.)) / (map_builder.size())
                                                + Vec2::new(-0.5, 0.5),
                                        )
                                        .with_scale(Vec2::new(
                                            7. / map_builder.size().x,
                                            7. / map_builder.size().y,
                                        ))
                                        .with_depth(DEPTH_LAYER_MAP_LABEL),
                                    );
                            }
                            parent
                                .spawn(SpriteBundle {
                                    texture: asset_library.sprite_map_icon_boat.clone(),
                                    ..Default::default()
                                })
                                .insert(
                                    Transform2::from_xy(99999., 99999.)
                                        .with_depth(DEPTH_LAYER_MAP_PLAYER)
                                        .with_scale(Vec2::ONE * (6. / map_builder.size().x))
                                        .without_pixel_perfect(),
                                )
                                .insert(MapPlayer);
                            parent
                                .spawn(SpriteBundle {
                                    texture: asset_library.sprite_map_icon_quest.clone(),
                                    ..Default::default()
                                })
                                .insert(
                                    Transform2::from_xy(99999., 99999.)
                                        .with_depth(DEPTH_LAYER_MAP_OBJECTIVE)
                                        .with_scale(Vec2::ZERO)
                                        .without_pixel_perfect(),
                                )
                                .insert(MapObjective {
                                    scale: 5.5 / map_builder.size().x,
                                });
                        });
                });
        });
}

fn map_wait_for_close(
    input: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    mut ev_continue: EventWriter<CutsceneContinueEvent<MapCutscene>>,
    asset_library: Res<AssetLibrary>,
    mut commands: Commands,
) {
    if input.just_pressed(KeyCode::Space)
        || mouse.just_pressed(MouseButton::Left)
        || input.just_pressed(KeyCode::M)
    {
        ev_continue.send_default();

        commands
            .spawn(Transform2Bundle::default())
            .insert(
                AudioPlusSource::new(asset_library.sound_effects.sfx_overworld_map_close.clone())
                    .as_playing(),
            )
            .insert(TimeToLive { seconds: 3. });
    }
}

fn map_close(query: Query<Entity, With<Map>>, mut commands: Commands) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

#[derive(Component)]
pub struct Map;

#[derive(Component)]
pub struct MapPlayer;

#[derive(Component)]
pub struct MapObjective {
    scale: f32,
}

fn map_input(
    input: Res<Input<KeyCode>>,
    cutscenes: Res<Cutscenes>,
    state_time: Res<StateTime<AppState>>,
    mut ev_cutscene: EventWriter<CutsceneStartEvent<MapCutscene>>,
    game_state: Res<GameState>,
) {
    if state_time.time > 1.
        && input.just_pressed(KeyCode::M)
        && !cutscenes.running()
        && game_state.dangerous_seas
        && !game_state.quests.fighting()
    {
        ev_cutscene.send_default();
    }
}

fn map_update_player(
    player_query: Query<&GlobalTransform, With<Player>>,
    mut query: Query<&mut Transform2, With<MapPlayer>>,
    map_builder: Res<MapBuilder>,
) {
    let player_position = if let Ok(player_transform) = player_query.get_single() {
        player_transform.translation().truncate()
    } else {
        Vec2::new(99999., 99999.)
    };
    for mut map_player_transform in query.iter_mut() {
        map_player_transform.translation = map_builder.world_to_map(player_position);
    }
}

fn map_update_objective(
    mut query: Query<(&mut Transform2, &MapObjective)>,
    map_builder: Res<MapBuilder>,
    game_state: Res<GameState>,
    world_locations: Res<WorldLocations>,
    time: Res<Time>,
) {
    let objective_position = if let Some(objective_marker) = game_state.quests.marker() {
        world_locations.get_single_position(objective_marker) + Vec2::new(0., 300.)
    } else {
        Vec2::new(99999., 99999.)
    };
    for (mut map_player_transform, map_objective) in query.iter_mut() {
        map_player_transform.translation = map_builder.world_to_map(objective_position);
        map_player_transform.scale = Vec2::ONE * map_objective.scale
            + (map_objective.scale * 0.1 * (time.elapsed_seconds() * 1.5).cos());
    }
}
