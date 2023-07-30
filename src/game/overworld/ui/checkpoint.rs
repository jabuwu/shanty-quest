use crate::common::prelude::*;
use bevy::{prelude::*, sprite::Anchor};

const CHECKPOINT_POSITION: Vec2 = Vec2::new(0., 220.);

pub struct CheckpointPlugin;

impl Plugin for CheckpointPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CheckpointSpawnEvent>()
            .add_systems(Update, checkpoint_spawn);
    }
}

#[derive(Event, Default, Clone, Copy)]
pub struct CheckpointSpawnEvent;

fn checkpoint_spawn(
    mut ev_spawn: EventReader<CheckpointSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for _ in ev_spawn.iter() {
        commands
            .spawn((
                VisibilityBundle::default(),
                TransformBundle::default(),
                FollowCamera { offset: Vec2::ZERO },
                Transform2::new().without_pixel_perfect(),
                TimeToLive { seconds: 1.5 },
            ))
            .with_children(|parent| {
                parent.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Vec2::new(400., 90.).into(),
                            color: Color::rgba(0., 0., 0., 0.36),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Transform2::from_translation(CHECKPOINT_POSITION)
                        .with_depth(DEPTH_LAYER_CHECKPOINT_BACKGROUND)
                        .without_pixel_perfect(),
                ));
                parent.spawn((
                    Text2dBundle {
                        text: Text::from_section(
                            "Checkpoint",
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
                    Transform2::from_translation(CHECKPOINT_POSITION)
                        .with_depth(DEPTH_LAYER_CHECKPOINT_TEXT),
                ));
            });
    }
}
