use crate::common::prelude::*;
use audio_plus::prelude::*;
use bevy::prelude::*;

#[derive(Default)]
struct LevelUpState {
    last_spawn_time: f32,
}

const LEVEL_UP_POSITION: Vec2 = Vec2::new(0., 220.);

pub struct LevelUpPlugin;

impl Plugin for LevelUpPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LevelUpState>()
            .add_event::<LevelUpSpawnEvent>()
            .add_system(level_up_spawn);
    }
}

#[derive(Default, Clone, Copy)]
pub struct LevelUpSpawnEvent;

fn level_up_spawn(
    mut ev_spawn: EventReader<LevelUpSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
    time: Res<Time>,
    mut state: ResMut<LevelUpState>,
) {
    for _ in ev_spawn.iter() {
        if time.time_since_startup().as_secs_f32() > state.last_spawn_time + 4.5 {
            state.last_spawn_time = time.time_since_startup().as_secs_f32();
            commands
                .spawn_bundle(VisibilityBundle::default())
                .insert_bundle(TransformBundle::default())
                .insert(FollowCamera { offset: Vec2::ZERO })
                .insert(Transform2::new().without_pixel_perfect())
                .insert(TimeToLive { seconds: 4.5 })
                .insert(
                    AudioPlusSource::new(
                        asset_library.sound_effects.sfx_overworld_level_up.clone(),
                    )
                    .as_playing(),
                )
                .with_children(|parent| {
                    parent
                        .spawn_bundle(SpriteBundle {
                            sprite: Sprite {
                                custom_size: Vec2::new(400., 110.).into(),
                                color: Color::rgba(0., 0., 0., 0.36),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(
                            Transform2::from_translation(LEVEL_UP_POSITION)
                                .with_depth(DEPTH_LAYER_LEVEL_UP_BACKGROUND)
                                .without_pixel_perfect(),
                        );
                    parent
                        .spawn_bundle(Text2dBundle {
                            text: Text::from_section(
                                "Level Up",
                                TextStyle {
                                    font: asset_library.font_bold.clone(),
                                    font_size: 62.0,
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
                            Transform2::from_translation(LEVEL_UP_POSITION + Vec2::new(0., 11.))
                                .with_depth(DEPTH_LAYER_LEVEL_UP_TEXT),
                        );
                    parent
                        .spawn_bundle(Text2dBundle {
                            text: Text::from_section(
                                "Spend skill points at town",
                                TextStyle {
                                    font: asset_library.font_bold.clone(),
                                    font_size: 22.0,
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
                            Transform2::from_translation(LEVEL_UP_POSITION + Vec2::new(0., -21.))
                                .with_depth(DEPTH_LAYER_LEVEL_UP_TEXT),
                        );
                });
        }
    }
}
