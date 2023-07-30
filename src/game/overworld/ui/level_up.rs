use crate::common::prelude::*;
use audio_plus::prelude::*;
use bevy::{prelude::*, sprite::Anchor};

#[derive(Default, Resource)]
struct LevelUpState {
    last_spawn_time: f32,
}

const LEVEL_UP_POSITION: Vec2 = Vec2::new(0., 220.);

pub struct LevelUpPlugin;

impl Plugin for LevelUpPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LevelUpState>()
            .add_event::<LevelUpSpawnEvent>()
            .add_systems(Update, level_up_spawn);
    }
}

#[derive(Event, Default, Clone, Copy)]
pub struct LevelUpSpawnEvent;

fn level_up_spawn(
    mut ev_spawn: EventReader<LevelUpSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
    time: Res<Time>,
    mut state: ResMut<LevelUpState>,
) {
    for _ in ev_spawn.iter() {
        if time.elapsed_seconds() > state.last_spawn_time + 4.5 {
            state.last_spawn_time = time.elapsed_seconds();
            commands
                .spawn((
                    VisibilityBundle::default(),
                    TransformBundle::default(),
                    FollowCamera { offset: Vec2::ZERO },
                    Transform2::new().without_pixel_perfect(),
                    TimeToLive { seconds: 4.5 },
                    AudioPlusSource::new(
                        asset_library.sound_effects.sfx_overworld_level_up.clone(),
                    )
                    .as_playing(),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        SpriteBundle {
                            sprite: Sprite {
                                custom_size: Vec2::new(400., 110.).into(),
                                color: Color::rgba(0., 0., 0., 0.36),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        Transform2::from_translation(LEVEL_UP_POSITION)
                            .with_depth(DEPTH_LAYER_LEVEL_UP_BACKGROUND)
                            .without_pixel_perfect(),
                    ));
                    parent.spawn((
                        Text2dBundle {
                            text: Text::from_section(
                                "Level Up",
                                TextStyle {
                                    font: asset_library.font_bold.clone(),
                                    font_size: 62.0,
                                    color: Color::WHITE,
                                },
                            )
                            .with_alignment(TextAlignment::Center),
                            text_anchor: Anchor::Center,
                            ..Default::default()
                        },
                        Transform2::from_translation(LEVEL_UP_POSITION + Vec2::new(0., 11.))
                            .with_depth(DEPTH_LAYER_LEVEL_UP_TEXT),
                    ));
                    parent.spawn((
                        Text2dBundle {
                            text: Text::from_section(
                                "Spend skill points at town",
                                TextStyle {
                                    font: asset_library.font_bold.clone(),
                                    font_size: 22.0,
                                    color: Color::WHITE,
                                },
                            )
                            .with_alignment(TextAlignment::Center),
                            text_anchor: Anchor::Center,
                            ..Default::default()
                        },
                        Transform2::from_translation(LEVEL_UP_POSITION + Vec2::new(0., -21.))
                            .with_depth(DEPTH_LAYER_LEVEL_UP_TEXT),
                    ));
                });
        }
    }
}
