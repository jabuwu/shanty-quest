use crate::common::prelude::*;
use bevy::prelude::*;

use self::boat_preview::BoatPreviewSpawnEvent;
use self::upgrades::UpgradesSpawnEvent;

pub struct ConcertHallPlugin;

impl Plugin for ConcertHallPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(band_selection::BandSelectionPlugin)
            .add_plugin(boat_preview::BoatPreviewPlugin)
            .add_plugin(upgrades::UpgradesPlugin)
            .add_system_set(
                SystemSet::on_enter(AppState::TownConcertHall).with_system(concert_hall_init),
            )
            .add_system_set(
                SystemSet::on_update(AppState::TownConcertHall).with_system(concert_hall_leave),
            );
    }
}

fn concert_hall_init(
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
    mut ev_upgrades_spawn: EventWriter<UpgradesSpawnEvent>,
    mut ev_boat_preview_spawn: EventWriter<BoatPreviewSpawnEvent>,
) {
    commands.spawn_bundle(Camera2dBundle::default());
    ev_upgrades_spawn.send_default();
    ev_boat_preview_spawn.send_default();
    /*commands
    .spawn_bundle(SpriteBundle {
        texture: asset_library.sprite_town_bg_hole.clone(),
        ..Default::default()
    })
    .insert(
        Transform2::new()
            .with_depth((DepthLayer::Front, 0.))
            .with_scale(Vec2::ONE * 0.5),
    );*/
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    position_type: PositionType::Relative,
                    position: UiRect {
                        top: Val::Px(300.),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                text: Text::from_section(
                    "Press space to exit".to_owned(),
                    TextStyle {
                        font: asset_library.font_default.clone(),
                        font_size: 42.0,
                        color: Color::WHITE,
                    },
                )
                .with_alignment(TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    vertical: VerticalAlign::Center,
                }),
                ..Default::default()
            });
        });
}

fn concert_hall_leave(mut keys: ResMut<Input<KeyCode>>, mut app_state: ResMut<State<AppState>>) {
    if keys.just_pressed(KeyCode::Space) {
        app_state.set(AppState::TownOutside).unwrap();
        keys.reset(KeyCode::Space);
    }
}

pub mod band_selection;
pub mod boat_preview;
pub mod upgrades;
