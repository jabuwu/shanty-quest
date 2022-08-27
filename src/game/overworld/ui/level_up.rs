use crate::common::prelude::*;
use bevy::prelude::*;

const LEVEL_UP_POSITION: Vec2 = Vec2::new(0., 220.);

pub struct LevelUpPlugin;

impl Plugin for LevelUpPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LevelUpSpawnEvent>()
            .add_system(level_up_spawn);
    }
}

#[derive(Default, Clone, Copy)]
pub struct LevelUpSpawnEvent;

fn level_up_spawn(
    mut ev_spawn: EventReader<LevelUpSpawnEvent>,
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
                        Transform2::from_translation(LEVEL_UP_POSITION)
                            .with_depth(DEPTH_LAYER_LEVEL_UP_TEXT),
                    );
            });
    }
}
