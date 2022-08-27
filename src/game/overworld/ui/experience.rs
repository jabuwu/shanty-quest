use crate::common::prelude::*;
use bevy::prelude::*;

const EXPERIENCE_UI_POSITION: Vec2 = Vec2::new(-435., -355.);
const EXPERIENCE_UI_SCALE: f32 = 0.28;

const EXPERIENCE_UI_LEVEL_LABEL_POSITION: Vec2 = Vec2::new(-565., -45.);
const EXPERIENCE_UI_LEVEL_LABEL_FONT_SIZE: f32 = 75.;

const EXPERIENCE_UI_LEVEL_POSITION: Vec2 = Vec2::new(-477., -60.);
const EXPERIENCE_UI_LEVEL_FONT_SIZE: f32 = 110.;

const EXPERIENCE_UI_SKILLPOINT_POSITION: Vec2 = Vec2::new(-130., 430.);
const EXPERIENCE_UI_SKILLPOINT_BG_SIZE: f32 = 1.2;
const EXPERIENCE_UI_SKILLPOINT_TEXT_FONT_SIZE: f32 = 68.;

pub struct ExperienceUiPlugin;

impl Plugin for ExperienceUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ExperienceUiSpawnEvent>()
            .add_system(controls_ui_spawn);
    }
}

#[derive(Default, Clone, Copy)]
pub struct ExperienceUiSpawnEvent;

fn controls_ui_spawn(
    mut ev_spawn: EventReader<ExperienceUiSpawnEvent>,
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
                        transform2: Transform2::from_translation(EXPERIENCE_UI_POSITION)
                            .with_scale(Vec2::ONE * EXPERIENCE_UI_SCALE),
                        ..Default::default()
                    })
                    .insert_bundle(VisibilityBundle::default())
                    .with_children(|parent| {
                        parent
                            .spawn_bundle(SpriteBundle {
                                texture: asset_library.sprite_experience_bar_bg.clone(),
                                ..Default::default()
                            })
                            .insert(
                                Transform2::from_xy(7., -5.)
                                    .with_depth(DEPTH_LAYER_UI_EXPERIENCE_BAR_BACK),
                            );
                        parent
                            .spawn_bundle(SpriteBundle {
                                sprite: Sprite {
                                    custom_size: Vec2::new(586., 50.).into(),
                                    color: Color::rgb_u8(255, 209, 22),
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .insert(
                                Transform2::from_xy(0., 0.)
                                    .with_depth(DEPTH_LAYER_UI_EXPERIENCE_BAR),
                            );
                        parent
                            .spawn_bundle(Text2dBundle {
                                text: Text::from_section(
                                    "Lvl",
                                    TextStyle {
                                        font: asset_library.font_bold.clone(),
                                        font_size: EXPERIENCE_UI_LEVEL_LABEL_FONT_SIZE,
                                        color: Color::WHITE,
                                    },
                                )
                                .with_alignment(TextAlignment {
                                    horizontal: HorizontalAlign::Left,
                                    vertical: VerticalAlign::Bottom,
                                }),
                                ..Default::default()
                            })
                            .insert(
                                Transform2::from_translation(EXPERIENCE_UI_LEVEL_LABEL_POSITION)
                                    .with_depth(DEPTH_LAYER_UI_EXPERIENCE_LEVEL),
                            );
                        parent
                            .spawn_bundle(Text2dBundle {
                                text: Text::from_section(
                                    "25",
                                    TextStyle {
                                        font: asset_library.font_bold.clone(),
                                        font_size: EXPERIENCE_UI_LEVEL_FONT_SIZE,
                                        color: Color::WHITE,
                                    },
                                )
                                .with_alignment(TextAlignment {
                                    horizontal: HorizontalAlign::Left,
                                    vertical: VerticalAlign::Bottom,
                                }),
                                ..Default::default()
                            })
                            .insert(
                                Transform2::from_translation(EXPERIENCE_UI_LEVEL_POSITION)
                                    .with_depth(DEPTH_LAYER_UI_EXPERIENCE_LEVEL),
                            );
                        parent
                            .spawn_bundle(SpriteBundle {
                                texture: asset_library.sprite_experience_skill_point_bg.clone(),
                                ..Default::default()
                            })
                            .insert(
                                Transform2::from_translation(EXPERIENCE_UI_SKILLPOINT_POSITION)
                                    .with_depth(DEPTH_LAYER_UI_EXPERIENCE_SKILLPOINT_BG)
                                    .with_scale(Vec2::ONE * EXPERIENCE_UI_SKILLPOINT_BG_SIZE),
                            );
                        parent
                            .spawn_bundle(Text2dBundle {
                                text: Text::from_section(
                                    "5 Skill Points to spend!",
                                    TextStyle {
                                        font: asset_library.font_bold.clone(),
                                        font_size: EXPERIENCE_UI_SKILLPOINT_TEXT_FONT_SIZE,
                                        color: Color::WHITE,
                                    },
                                )
                                .with_alignment(TextAlignment {
                                    horizontal: HorizontalAlign::Center,
                                    vertical: VerticalAlign::Center,
                                }),
                                ..Default::default()
                            })
                            .insert(
                                Transform2::from_translation(
                                    EXPERIENCE_UI_SKILLPOINT_POSITION + Vec2::new(0., -10.),
                                )
                                .with_depth(DEPTH_LAYER_UI_EXPERIENCE_SKILLPOINT_TEXT),
                            );
                    });
            });
    }
}
