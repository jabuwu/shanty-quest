use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;

use self::boat_preview::BoatPreviewSpawnEvent;
use self::upgrades::UpgradesSpawnEvent;

#[derive(Default)]
pub struct ConcertHallState {
    leave: bool,
}

pub struct ConcertHallPlugin;

impl Plugin for ConcertHallPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ConcertHallState>()
            .add_plugin(band_selection::BandSelectionPlugin)
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
    mut game_state: ResMut<GameState>,
    mut dialogue: ResMut<Dialogue>,
    mut screen_fade: ResMut<ScreenFade>,
    mut state: ResMut<ConcertHallState>,
) {
    *state = ConcertHallState::default();
    commands.spawn_bundle(Camera2dBundle::default());
    ev_upgrades_spawn.send_default();
    ev_boat_preview_spawn.send_default();
    screen_fade.fade_in(1.);
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
        .spawn_bundle(Text2dBundle {
            text: Text::from_section(
                "Press ESC to exit",
                TextStyle {
                    font: asset_library.font_bold.clone(),
                    font_size: 42.0,
                    color: Color::rgb_u8(52, 52, 52),
                },
            )
            .with_alignment(TextAlignment {
                horizontal: HorizontalAlign::Center,
                vertical: VerticalAlign::Center,
            }),
            ..Default::default()
        })
        .insert(Transform2::from_xy(0., -320.).with_depth(DEPTH_LAYER_UPGRADES_LEAVE_TEXT));
    if !game_state.quests.upgrades_dialogue {
        for (p, t) in UPGRADE_MENU.iter() {
            dialogue.add_text(*p, String::from(*t));
        }
        game_state.quests.upgrades_dialogue = true;
    }
}

fn concert_hall_leave(
    keys: Res<Input<KeyCode>>,
    mut app_state: ResMut<State<AppState>>,
    mut state: ResMut<ConcertHallState>,
    mut screen_fade: ResMut<ScreenFade>,
) {
    if !state.leave && keys.just_pressed(KeyCode::Escape) {
        state.leave = true;
        screen_fade.fade_out(1.);
    }
    if screen_fade.faded_out() && state.leave {
        app_state.set(AppState::TownOutside).unwrap();
    }
}

pub mod band_selection;
pub mod boat_preview;
pub mod upgrades;
