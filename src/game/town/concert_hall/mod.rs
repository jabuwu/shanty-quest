use crate::common::prelude::*;
use crate::game::prelude::*;
use audio_plus::prelude::*;
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

#[derive(Component)]
struct Leave;

#[derive(Component)]
struct ClickSound;

#[derive(Component)]
struct HoverSound;

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
                "Back to Town".to_owned(),
                TextStyle {
                    font: asset_library.font_bold.clone(),
                    font_size: 64.0,
                    color: Color::BLACK,
                },
            )
            .with_alignment(TextAlignment {
                horizontal: HorizontalAlign::Center,
                vertical: VerticalAlign::Center,
            }),
            ..Default::default()
        })
        .insert(Clickable::new(CollisionShape::Rect {
            size: Vec2::new(350., 150.),
        }))
        .insert(Transform2::from_xy(0., -320.).with_depth(DEPTH_LAYER_UPGRADES_LEAVE_TEXT))
        .insert(Leave);
    commands
        .spawn()
        .insert(AudioPlusSource::new(
            asset_library.sound_effects.sfx_town_outside_click.clone(),
        ))
        .insert(ClickSound);
    commands
        .spawn()
        .insert(AudioPlusSource::new(
            asset_library.sound_effects.sfx_town_outside_hover.clone(),
        ))
        .insert(HoverSound);
    if !game_state.quests.upgrades_dialogue {
        for (p, t) in UPGRADE_MENU.iter() {
            dialogue.add_text(*p, String::from(*t));
        }
        game_state.quests.upgrades_dialogue = true;
    }
}

fn concert_hall_leave(
    mut query: Query<(&mut Text, &Clickable), With<Leave>>,
    mut app_state: ResMut<State<AppState>>,
    mut state: ResMut<ConcertHallState>,
    mut screen_fade: ResMut<ScreenFade>,
    mut sound_query: ParamSet<(
        Query<&mut AudioPlusSource, With<HoverSound>>,
        Query<&mut AudioPlusSource, With<ClickSound>>,
    )>,
    dialogue: Res<Dialogue>,
) {
    let block_input = state.leave || dialogue.visible();
    for (mut text, clickable) in query.iter_mut() {
        if clickable.just_hovered() && !block_input {
            for mut source in sound_query.p0().iter_mut() {
                source.play();
            }
        }
        if clickable.just_clicked() && !block_input {
            for mut source in sound_query.p1().iter_mut() {
                source.play();
            }
        }
        text.sections[0].style.color = if (clickable.hovered && !block_input) || state.leave {
            Color::WHITE
        } else {
            Color::BLACK
        };
        if !block_input && clickable.confirmed {
            state.leave = true;
            screen_fade.fade_out(1.);
        }
    }
    if screen_fade.faded_out() && state.leave {
        app_state.set(AppState::TownOutside).unwrap();
    }
}

pub mod band_selection;
pub mod boat_preview;
pub mod upgrades;
