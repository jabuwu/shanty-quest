use crate::common::prelude::*;
use bevy::prelude::*;

const CHECKPOINT_POSITION: Vec2 = Vec2::new(0., 220.);

pub struct CheckpointPlugin;

impl Plugin for CheckpointPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CheckpointSpawnEvent>()
            .add_system(checkpoint_spawn);
    }
}

#[derive(Default, Clone, Copy)]
pub struct CheckpointSpawnEvent;

fn checkpoint_spawn(
    mut ev_spawn: EventReader<CheckpointSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for _ in ev_spawn.iter() {
        commands
            .spawn_bundle(VisibilityBundle::default())
            .insert_bundle(TransformBundle::default())
            .insert(FollowCamera { offset: Vec2::ZERO })
            .insert(Transform2::new().without_pixel_perfect())
            .insert(TimeToLive { seconds: 1.5 })
            .with_children(|parent| {
                parent
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            custom_size: Vec2::new(400., 90.).into(),
                            color: Color::rgba(0., 0., 0., 0.36),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(
                        Transform2::from_translation(CHECKPOINT_POSITION)
                            .with_depth(DEPTH_LAYER_CHECKPOINT_BACKGROUND)
                            .without_pixel_perfect(),
                    );
                parent
                    .spawn_bundle(Text2dBundle {
                        text: Text::from_section(
                            "Checkpoint",
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
                        Transform2::from_translation(CHECKPOINT_POSITION)
                            .with_depth(DEPTH_LAYER_CHECKPOINT_TEXT),
                    );
            });
    }
}
