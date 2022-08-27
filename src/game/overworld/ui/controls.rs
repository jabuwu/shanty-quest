use crate::{common::prelude::*, game::prelude::*};
use bevy::prelude::*;

const CONTROLS_UI_POSITION: Vec2 = Vec2::new(-260., -315.);
const CONTROLS_UI_SCALE: f32 = 0.55;

pub struct ControlsUiPlugin;

impl Plugin for ControlsUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ControlsUiSpawnEvent>()
            .add_system(controls_ui_spawn)
            .add_system(controls_ui_update_dash)
            .add_system(controls_ui_update_map);
    }
}

#[derive(Default, Clone, Copy)]
pub struct ControlsUiSpawnEvent;

#[derive(Component)]
pub struct ControlsUiDash;

#[derive(Component)]
pub struct ControlsUiMap;

fn controls_ui_spawn(
    mut ev_spawn: EventReader<ControlsUiSpawnEvent>,
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
                    .spawn_bundle(Transform2Bundle {
                        transform2: Transform2::from_translation(CONTROLS_UI_POSITION)
                            .with_scale(Vec2::ONE * CONTROLS_UI_SCALE),
                        ..Default::default()
                    })
                    .insert_bundle(VisibilityBundle::default())
                    .with_children(|parent| {
                        parent
                            .spawn_bundle(SpriteBundle {
                                texture: asset_library.sprite_controls_dash.clone(),
                                ..Default::default()
                            })
                            .insert(
                                Transform2::from_xy(0., 0.)
                                    .with_scale(Vec2::ONE * 0.5)
                                    .with_depth(DEPTH_LAYER_UI_CONTROLS),
                            )
                            .insert(ControlsUiDash);
                        parent
                            .spawn_bundle(SpriteBundle {
                                texture: asset_library.sprite_controls_map.clone(),
                                visibility: Visibility { is_visible: false },
                                ..Default::default()
                            })
                            .insert(
                                Transform2::from_xy(240., 0.)
                                    .with_scale(Vec2::ONE * 0.5)
                                    .with_depth(DEPTH_LAYER_UI_CONTROLS),
                            )
                            .insert(ControlsUiMap);
                    });
            });
    }
}

pub fn controls_ui_update_dash(
    mut query: Query<&mut Sprite, With<ControlsUiDash>>,
    player_query: Query<&Boat, With<Player>>,
) {
    let player_dash_cooldown = if let Ok(boat) = player_query.get_single() {
        boat.dash_cooldown > 0.
    } else {
        false
    };
    for mut sprite in query.iter_mut() {
        sprite
            .color
            .set_a(if player_dash_cooldown { 0.5 } else { 1. });
    }
}

pub fn controls_ui_update_map(
    mut query: Query<&mut Visibility, With<ControlsUiMap>>,
    game_state: Res<GameState>,
) {
    for mut visibility in query.iter_mut() {
        visibility.is_visible = game_state.dangerous_seas;
    }
}
