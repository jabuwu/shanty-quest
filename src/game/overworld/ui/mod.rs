use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;

use self::marker::MarkerSpawnEvent;
use self::objective::ObjectiveSpawnEvent;
use self::town_marker::TownMarkerSpawnEvent;

pub struct OverworldUiPlugin;

impl Plugin for OverworldUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OverworldUiSpawnEvent>()
            .add_plugin(map::MapPlugin)
            .add_plugin(marker::MarkerPlugin)
            .add_plugin(town_marker::TownMarkerPlugin)
            .add_plugin(objective::ObjectivePlugin)
            .add_plugin(boss_healthbar::BossHealthbarPlugin)
            .add_plugin(checkpoint::CheckpointPlugin)
            .add_system(overworld_ui_spawn)
            .add_system(overworld_ui_health);
    }
}

#[derive(Default, Clone, Copy)]
pub struct OverworldUiSpawnEvent;

#[derive(Component)]
pub struct OverworldUiHealth;

fn overworld_ui_spawn(
    mut ev_spawn: EventReader<OverworldUiSpawnEvent>,
    mut commands: Commands,
    mut ev_marker_spawn: EventWriter<MarkerSpawnEvent>,
    mut ev_town_marker_spawn: EventWriter<TownMarkerSpawnEvent>,
    mut ev_objective_spawn: EventWriter<ObjectiveSpawnEvent>,
    mut ev_checkpoint_spawn: EventWriter<CheckpointSpawnEvent>,
    asset_library: Res<AssetLibrary>,
    game_state: Res<GameState>,
) {
    for _ in ev_spawn.iter() {
        ev_marker_spawn.send_default();
        ev_town_marker_spawn.send_default();
        ev_objective_spawn.send_default();
        if game_state.checkpoint_notification {
            ev_checkpoint_spawn.send_default();
        }
        commands
            .spawn_bundle(VisibilityBundle::default())
            .insert_bundle(TransformBundle::default())
            .insert(FollowCamera { offset: Vec2::ZERO })
            .insert(Transform2::new())
            .with_children(|parent| {
                parent
                    .spawn_bundle(Text2dBundle {
                        text: Text::from_section(
                            "",
                            TextStyle {
                                font: asset_library.font_default.clone(),
                                font_size: 48.0,
                                color: Color::BLACK,
                            },
                        )
                        .with_alignment(TextAlignment {
                            horizontal: HorizontalAlign::Left,
                            vertical: VerticalAlign::Bottom,
                        }),
                        ..Default::default()
                    })
                    .insert(Transform2::from_xy(-570., -340.).with_depth(DEPTH_LAYER_UI_TEXT))
                    .insert(OverworldUiHealth);
            });
    }
}

fn overworld_ui_health(
    mut query: Query<&mut Text, With<OverworldUiHealth>>,
    health_query: Query<&Health, With<Player>>,
    threat_level: Res<ThreatLevel>,
) {
    let health = if let Ok(health) = health_query.get_single() {
        health.value
    } else {
        0.
    };
    for mut text in query.iter_mut() {
        text.sections[0].value =
            format!("Rum: {}\nThreat level: {:?}", health, threat_level.as_ref());
    }
}

pub mod boss_healthbar;
pub mod checkpoint;
pub mod map;
pub mod marker;
pub mod objective;
pub mod town_marker;
