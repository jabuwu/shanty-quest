use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;

pub struct TownMarkerPlugin;

impl Plugin for TownMarkerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TownMarkerSpawnEvent>()
            .add_system(town_marker_spawn)
            .add_system(town_marker_update);
    }
}

#[derive(Default, Clone, Copy)]
pub struct TownMarkerSpawnEvent;

#[derive(Component)]
pub struct TownMarkerIcon;

#[derive(Component)]
pub struct TownMarkerArrow;

fn town_marker_spawn(
    mut ev_spawn: EventReader<TownMarkerSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for _ in ev_spawn.iter() {
        commands
            .spawn_bundle(VisibilityBundle::default())
            .insert_bundle(TransformBundle::default())
            .insert(FollowCamera { offset: Vec2::ZERO })
            .insert(Transform2::new().without_pixel_perfect())
            .with_children(|parent| {
                parent
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            ..Default::default()
                        },
                        texture: asset_library.sprite_world_town_marker_icon.clone(),
                        ..Default::default()
                    })
                    .insert(
                        Transform2::from_xy(0., 0.)
                            .with_depth(DEPTH_LAYER_UI_MARKER_ICON)
                            .with_scale(Vec2::ONE * 0.25)
                            .without_pixel_perfect(),
                    )
                    .insert(TownMarkerIcon)
                    .with_children(|parent| {
                        parent
                            .spawn_bundle(SpriteBundle {
                                sprite: Sprite {
                                    ..Default::default()
                                },
                                texture: asset_library.sprite_world_quest_marker_arrow.clone(),
                                ..Default::default()
                            })
                            .insert(TownMarkerArrow)
                            .insert(
                                Transform2::from_xy(0., 0.)
                                    .with_depth(DEPTH_LAYER_UI_MARKER_ARROW)
                                    .without_pixel_perfect(),
                            );
                    });
            });
    }
}

fn town_marker_update(
    mut queries: ParamSet<(
        Query<&GlobalTransform, With<Camera>>,
        Query<(&mut Transform2, &mut Sprite), With<TownMarkerIcon>>,
        Query<(&mut Transform2, &mut Sprite), With<TownMarkerArrow>>,
        Query<&GlobalTransform, With<Town>>,
    )>,
    game_state: Res<GameState>,
) {
    let camera_position = if let Ok(transform) = queries.p0().get_single() {
        transform.translation().truncate()
    } else {
        Vec2::ZERO
    };
    let mut closest_town = None;
    let mut closest_town_distance = 0.;
    for town_transform in queries.p3().iter() {
        let distance = camera_position.distance(town_transform.translation().truncate());
        if distance < closest_town_distance || closest_town.is_none() {
            closest_town = Some(town_transform.translation().truncate());
            closest_town_distance = distance;
        }
    }
    if game_state.quests.hide_town_marker() {
        closest_town = None;
    }
    if let Some(town_position) = closest_town {
        let difference = (town_position - camera_position).normalize_or_zero();
        let distance = town_position.distance(camera_position);
        let alpha = ((distance - 350.) / 300.).clamp(0., 1.);
        for (mut icon_transform, mut icon_sprite) in queries.p1().iter_mut() {
            icon_transform.translation = difference * 330.;
            icon_sprite.color.set_a(alpha);
        }
        for (mut arrow_transform, mut arrow_sprite) in queries.p2().iter_mut() {
            arrow_transform.rotation =
                Vec2::X.angle_between(difference) + std::f32::consts::PI * 0.5;
            arrow_sprite.color.set_a(alpha);
        }
    } else {
        for (_, mut icon_sprite) in queries.p1().iter_mut() {
            icon_sprite.color.set_a(0.);
        }
        for (_, mut arrow_sprite) in queries.p2().iter_mut() {
            arrow_sprite.color.set_a(0.);
        }
    }
}
