use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(map_update_player)
            .add_cutscene::<MapCutscene>()
            .add_system_set(SystemSet::on_update(AppState::Overworld).with_system(map_input));
    }
}

#[derive(Default, Debug, Clone)]
pub struct MapCutscene;

impl Cutscene for MapCutscene {
    fn build(cutscene: &mut CutsceneBuilder) {
        cutscene.add_step(map_open, map_wait_for_close);
        cutscene.add_quick_step(map_close);
    }
}

fn map_open(mut commands: Commands, map_builder: Res<MapBuilder>) {
    commands
        .spawn_bundle(VisibilityBundle::default())
        .insert_bundle(TransformBundle::default())
        .insert(Transform2::from_xy(0., 0.).without_pixel_perfect())
        .insert(FollowCamera { offset: Vec2::ZERO })
        .insert(Map)
        .with_children(|parent| {
            parent
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Vec2::new(1., 1.).into(),
                        color: Color::BEIGE,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(
                    Transform2::from_xy(0., 0.)
                        .with_scale(Vec2::new(700., 700.))
                        .with_depth(DEPTH_LAYER_MAP_BACK)
                        .without_pixel_perfect(),
                )
                .with_children(|parent| {
                    for tile in map_builder.tiles.iter() {
                        parent
                            .spawn_bundle(SpriteBundle {
                                sprite: Sprite {
                                    custom_size: Vec2::new(
                                        120. / map_builder.size().x,
                                        120. / map_builder.size().y,
                                    )
                                    .into(),
                                    color: Color::BLACK,
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .insert(
                                Transform2::from_translation(
                                    *tile / (map_builder.size()) + Vec2::new(-0.5, 0.5),
                                )
                                .with_depth(DEPTH_LAYER_MAP_TILE),
                            );
                    }
                    parent
                        .spawn_bundle(SpriteBundle {
                            sprite: Sprite {
                                custom_size: Vec2::new(
                                    (100. / map_builder.size().x) * 3.,
                                    (100. / map_builder.size().y) * 3.,
                                )
                                .into(),
                                color: Color::RED,
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(
                            Transform2::from_xy(99999., 99999.)
                                .with_depth(DEPTH_LAYER_MAP_PLAYER)
                                .without_pixel_perfect(),
                        )
                        .insert(MapPlayer);
                });
        });
}

fn map_wait_for_close(
    input: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    mut ev_continue: EventWriter<CutsceneContinueEvent<MapCutscene>>,
) {
    if input.just_pressed(KeyCode::Space) || mouse.just_pressed(MouseButton::Left) {
        ev_continue.send_default();
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

fn map_input(
    input: Res<Input<KeyCode>>,
    cutscenes: Res<Cutscenes>,
    state_time: Res<StateTime<AppState>>,
    mut ev_cutscene: EventWriter<CutsceneStartEvent<MapCutscene>>,
) {
    if state_time.time > 1. && input.just_pressed(KeyCode::M) && !cutscenes.running() {
        ev_cutscene.send_default();
    }
}

fn map_update_player(
    player_query: Query<&GlobalTransform, With<Player>>,
    mut query: Query<&mut Transform2, With<MapPlayer>>,
    map_builder: Res<MapBuilder>,
) {
    let mut player_position = if let Ok(player_transform) = player_query.get_single() {
        player_transform.translation().truncate()
    } else {
        Vec2::new(99999., 99999.)
    };
    player_position -= map_builder.offset();
    player_position /= map_builder.size();
    player_position += Vec2::new(-0.5, 0.5);
    for mut map_player_transform in query.iter_mut() {
        map_player_transform.translation = player_position;
    }
}
