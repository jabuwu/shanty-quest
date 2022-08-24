use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;

pub struct OverworldUiPlugin;

impl Plugin for OverworldUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OverworldUiSpawnEvent>()
            .add_system(overworld_ui_spawn)
            .add_system(overworld_ui_health);
    }
}

#[derive(Default, Clone, Copy)]
pub struct OverworldUiSpawnEvent;

#[derive(Component)]
pub struct OverworldUiHealth;

fn overworld_ui_spawn(
    mut ev_spawn: EventReader<WorldLoadEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for _ in ev_spawn.iter() {
        commands
            .spawn_bundle(VisibilityBundle::default())
            .insert_bundle(TransformBundle::default())
            .insert(Transform2::from_xy(0., 0.).with_depth((DepthLayer::Front, 0.9)))
            .insert(FollowCamera { offset: Vec2::ZERO })
            .with_children(|parent| {
                parent
                    .spawn_bundle(Text2dBundle {
                        text: Text::from_section(
                            "",
                            TextStyle {
                                font: asset_library.font_default.clone(),
                                font_size: 64.0,
                                color: Color::BLACK,
                            },
                        )
                        .with_alignment(TextAlignment {
                            horizontal: HorizontalAlign::Left,
                            vertical: VerticalAlign::Bottom,
                        }),
                        ..Default::default()
                    })
                    .insert(Transform2::from_xy(-570., -340.).with_depth((DepthLayer::Front, 0.)))
                    .insert(OverworldUiHealth);
            });
    }
}

fn overworld_ui_health(
    mut query: Query<&mut Text, With<OverworldUiHealth>>,
    health_query: Query<&Health, With<Player>>,
) {
    let health = if let Ok(health) = health_query.get_single() {
        health.value
    } else {
        0.
    };
    for mut text in query.iter_mut() {
        text.sections[0].value = format!("Rum: {}", health);
    }
}
