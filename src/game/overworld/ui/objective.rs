use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;

pub struct ObjectivePlugin;

impl Plugin for ObjectivePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ObjectiveSpawnEvent>()
            .add_system(objective_spawn)
            .add_system(objective_update);
    }
}

#[derive(Default, Clone, Copy)]
pub struct ObjectiveSpawnEvent;

#[derive(Component)]
pub struct ObjectiveBackground;

#[derive(Component)]
pub struct ObjectiveHud;

#[derive(Component)]
pub struct ObjectiveText;

fn objective_spawn(
    mut ev_spawn: EventReader<ObjectiveSpawnEvent>,
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
                            custom_size: Vec2::new(1., 90.).into(),
                            color: Color::rgba(0., 0., 0., 0.36),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(
                        Transform2::from_xy(625., 320.)
                            .with_depth(DEPTH_LAYER_UI_OBJECTIVE_BACKGROUND)
                            .without_pixel_perfect(),
                    )
                    .insert(ObjectiveBackground);
                parent
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            ..Default::default()
                        },
                        texture: asset_library.sprite_objective_bg.clone(),
                        ..Default::default()
                    })
                    .insert(
                        Transform2::from_xy(530., 335.)
                            .with_depth(DEPTH_LAYER_UI_OBJECTIVE_TEXT)
                            .without_pixel_perfect(),
                    )
                    .insert(ObjectiveHud);
                parent
                    .spawn_bundle(Text2dBundle {
                        text: Text::from_section(
                            "",
                            TextStyle {
                                font: asset_library.font_bold.clone(),
                                font_size: 28.0,
                                color: Color::WHITE,
                            },
                        )
                        .with_alignment(TextAlignment {
                            horizontal: HorizontalAlign::Right,
                            vertical: VerticalAlign::Top,
                        }),
                        ..Default::default()
                    })
                    .insert(
                        Transform2::from_xy(616., 312.).with_depth(DEPTH_LAYER_UI_OBJECTIVE_TEXT),
                    )
                    .insert(ObjectiveText);
            });
    }
}

fn objective_update(
    mut background_query: Query<&mut Transform2, With<ObjectiveBackground>>,
    mut hud_query: Query<&mut Visibility, With<ObjectiveHud>>,
    mut text_query: Query<&mut Text, With<ObjectiveText>>,
    game_state: Res<GameState>,
) {
    if let Some((width, str)) = game_state.quests.objective() {
        for mut background_transform in background_query.iter_mut() {
            background_transform.scale.x = width;
            background_transform.translation.x = 625. - width * 0.5;
        }
        for mut hud_visibility in hud_query.iter_mut() {
            hud_visibility.is_visible = true;
        }
        for mut text in text_query.iter_mut() {
            if text.sections[0].value != str {
                text.sections[0].value = str.to_owned();
            }
        }
    } else {
        for mut background_transform in background_query.iter_mut() {
            background_transform.scale.x = 0.;
        }
        for mut hud_visibility in hud_query.iter_mut() {
            hud_visibility.is_visible = false;
        }
        for mut text in text_query.iter_mut() {
            if text.sections[0].value != "" {
                text.sections[0].value = "".to_owned();
            }
        }
    }
}
