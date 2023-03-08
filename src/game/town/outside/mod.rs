use crate::common::{label::Label, prelude::*};
use crate::game::prelude::*;
use audio_plus::prelude::*;
use bevy::prelude::*;
use bevy::sprite::Anchor;

use super::TownAmbience;

#[derive(Default, Resource)]
pub struct OutsideState {
    leave: OutsideLeave,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum OutsideLeave {
    #[default]
    Stay,
    LeaveToOverworld,
    LeaveToConcertHall,
}

pub struct OutsidePlugin;

impl Plugin for OutsidePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<OutsideState>()
            .add_plugin(rum_refill::RumRefillPlugin)
            .add_system(outside_init.in_schedule(OnEnter(AppState::TownOutside)))
            .add_system(outside_leave.in_set(OnUpdate(AppState::TownOutside)))
            .add_system(outside_click)
            .add_system(outside_pulsing_icons)
            .add_system(outside_tavern_icon)
            .add_system(outside_mayor_icon)
            .add_system(outside_concert_hall_icon);
    }
}

#[derive(Component)]
struct ClickableItem {
    click_priority: i32,
    action: ClickableAction,
    last_hover: bool,
}

#[derive(PartialEq, Eq, Debug)]
enum ClickableAction {
    Tavern,
    Mayor,
    ConcertHall,
    Leave,
}

#[derive(Component)]
struct HoverSound;

#[derive(Component)]
struct ClickSound;

#[derive(Component)]
struct PulsingIcon {
    scale: Vec2,
}

#[derive(Component)]
struct TavernIcon;

#[derive(Component)]
struct MayorIcon;

#[derive(Component)]
struct ConcertHallIcon;

fn outside_init(
    mut state: ResMut<OutsideState>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
    mut game_state: ResMut<GameState>,
    mut dialogue: ResMut<Dialogue>,
    mut screen_fade: ResMut<ScreenFade>,
) {
    screen_fade.fade_in(0.5);
    *state = OutsideState::default();
    commands
        .spawn(Camera2dBundle::default())
        .insert(Transform2::new().with_depth((DepthLayer::Camera, 0.)));
    commands
        .spawn_empty()
        .insert(AudioPlusSource::new(
            asset_library.sound_effects.sfx_town_outside_hover.clone(),
        ))
        .insert(HoverSound);
    commands
        .spawn_empty()
        .insert(AudioPlusSource::new(
            asset_library.sound_effects.sfx_town_outside_click.clone(),
        ))
        .insert(ClickSound);
    commands
        .spawn(SpriteBundle {
            texture: asset_library.sprite_town_bg.clone(),
            ..Default::default()
        })
        .insert(
            Transform2::new()
                .with_depth(DEPTH_LAYER_TOWN_OUTSIDE_BG)
                .with_scale(Vec2::ONE * 0.5),
        );
    commands
        .spawn(VisibilityBundle {
            visibility: Visibility::Hidden,
            ..Default::default()
        })
        .insert(TransformBundle::default())
        .insert(Transform2::from_xy(-493., -84.))
        .insert(ClickableItem {
            click_priority: 1,
            action: ClickableAction::Tavern,
            last_hover: false,
        })
        .insert(Clickable::new(CollisionShape::Rect {
            size: Vec2::new(300., 350.),
        }))
        .with_children(|parent| {
            parent
                .spawn(SpriteBundle {
                    texture: asset_library.sprite_town_tavern_outline.clone(),
                    ..Default::default()
                })
                .insert(
                    Transform2::from_xy(0., 30.).with_depth(DEPTH_LAYER_TOWN_OUTSIDE_HIGHLIGHT),
                );
        });
    commands
        .spawn(VisibilityBundle {
            visibility: Visibility::Hidden,
            ..Default::default()
        })
        .insert(TransformBundle::default())
        .insert(Transform2::from_xy(369., 0.))
        .insert(ClickableItem {
            click_priority: 0,
            action: ClickableAction::Mayor,
            last_hover: false,
        })
        .insert(Clickable::new(CollisionShape::Rect {
            size: Vec2::new(550., 300.),
        }))
        .with_children(|parent| {
            parent
                .spawn(SpriteBundle {
                    texture: asset_library.sprite_town_mayor_outline.clone(),
                    ..Default::default()
                })
                .insert(
                    Transform2::from_xy(0., 43.).with_depth(DEPTH_LAYER_TOWN_OUTSIDE_HIGHLIGHT),
                );
        });
    commands
        .spawn(VisibilityBundle {
            visibility: Visibility::Hidden,
            ..Default::default()
        })
        .insert(TransformBundle::default())
        .insert(Transform2::from_xy(-229., 72.))
        .insert(ClickableItem {
            click_priority: 0,
            action: ClickableAction::ConcertHall,
            last_hover: false,
        })
        .insert(Clickable::new(CollisionShape::Rect {
            size: Vec2::new(500., 425.),
        }))
        .with_children(|parent| {
            parent
                .spawn(SpriteBundle {
                    texture: asset_library.sprite_town_concert_hall_outline.clone(),
                    ..Default::default()
                })
                .insert(
                    Transform2::from_xy(115., -5.).with_depth(DEPTH_LAYER_TOWN_OUTSIDE_HIGHLIGHT),
                );
        });
    commands
        .spawn(Text2dBundle {
            text: Text::from_section(
                "Exit Town".to_owned(),
                TextStyle {
                    font: asset_library.font_bold.clone(),
                    font_size: 64.0,
                    color: Color::BLACK,
                },
            )
            .with_alignment(TextAlignment::Center),
            text_anchor: Anchor::Center,
            ..Default::default()
        })
        .insert(Clickable::new(CollisionShape::Rect {
            size: Vec2::new(350., 150.),
        }))
        .insert(ClickableItem {
            click_priority: 0,
            action: ClickableAction::Leave,
            last_hover: false,
        })
        .insert(Transform2::from_xy(470., -330.).with_depth(DEPTH_LAYER_TOWN_OUTSIDE_EXIT));
    commands
        .spawn(Text2dBundle {
            text: Text::from_section(
                game_state.town.name.clone(),
                TextStyle {
                    font: asset_library.font_bold.clone(),
                    font_size: 64.0,
                    color: Color::BLACK,
                },
            )
            .with_alignment(TextAlignment::Center),
            text_anchor: Anchor::Center,
            ..Default::default()
        })
        .insert(Transform2::from_xy(0., 330.).with_depth(DEPTH_LAYER_TOWN_OUTSIDE_NAME));

    commands
        .spawn(SpriteBundle {
            texture: asset_library.sprite_town_tavern_notify.clone(),
            visibility: Visibility::Hidden,
            ..Default::default()
        })
        .insert(
            Transform2::from_xy(-554., 180.)
                .with_depth(DEPTH_LAYER_TOWN_OUTSIDE_ICON)
                .with_scale(Vec2::ONE * 0.5),
        )
        .insert(Label("Tavern Icon".to_owned()))
        .insert(TavernIcon)
        .insert(PulsingIcon {
            scale: Vec2::ONE * 0.5,
        });

    commands
        .spawn(SpriteBundle {
            texture: asset_library.sprite_town_mayor_notify.clone(),
            visibility: Visibility::Hidden,
            ..Default::default()
        })
        .insert(
            Transform2::from_xy(260., 180.)
                .with_depth(DEPTH_LAYER_TOWN_OUTSIDE_ICON)
                .with_scale(Vec2::ONE * 0.5),
        )
        .insert(Label("Mayor Icon".to_owned()))
        .insert(MayorIcon)
        .insert(PulsingIcon {
            scale: Vec2::ONE * 0.5,
        });

    commands
        .spawn(SpriteBundle {
            texture: asset_library.sprite_town_concert_hall_notify.clone(),
            visibility: Visibility::Hidden,
            ..Default::default()
        })
        .insert(
            Transform2::from_xy(-197., 311.)
                .with_depth(DEPTH_LAYER_TOWN_OUTSIDE_ICON)
                .with_scale(Vec2::ONE * 0.5),
        )
        .insert(Label("Concert Hall Icon".to_owned()))
        .insert(ConcertHallIcon)
        .insert(PulsingIcon {
            scale: Vec2::ONE * 0.5,
        });
    if game_state.quests.end() && !game_state.quests.endgame_town_dialogue {
        for (p, t) in JAGEROSSA_AFTER_VICTORY.iter() {
            dialogue.add_text(*p, String::from(*t));
        }
        game_state.quests.endgame_town_dialogue = true;
    }
}

fn outside_click(
    mut query: Query<(
        &mut Visibility,
        &Clickable,
        &mut ClickableItem,
        Option<&mut Text>,
    )>,
    mut input: ResMut<Input<MouseButton>>,
    mut screen_fade: ResMut<ScreenFade>,
    mut state: ResMut<OutsideState>,
    mut sound_query: ParamSet<(
        Query<&mut AudioPlusSource, With<HoverSound>>,
        Query<&mut AudioPlusSource, With<ClickSound>>,
        Query<&mut AudioPlusSource, With<TownAmbience>>,
    )>,
    state_time: Res<StateTime<AppState>>,
    mut dialogue: ResMut<Dialogue>,
    cutscenes: Res<Cutscenes>,
    game_state: Res<GameState>,
    mut ev_mayor_quest: EventWriter<QuestMayorEvent>,
    mut ev_barkeep_quest: EventWriter<QuestBarkeepEvent>,
) {
    if state_time.just_entered() || !matches!(state.leave, OutsideLeave::Stay) {
        return;
    }
    let mut highest_priority = -1;
    for (_, clickable, clickable_item, _) in query.iter_mut() {
        if clickable.hovered && clickable_item.click_priority > highest_priority {
            highest_priority = clickable_item.click_priority;
        }
    }
    for (mut visibility, clickable, mut clickable_item, mut text) in query.iter_mut() {
        if let Some(text) = text.as_mut() {
            text.sections[0].style.color = Color::BLACK;
        } else {
            *visibility = Visibility::Hidden;
        }
        let hovered = clickable.hovered
            && highest_priority == clickable_item.click_priority
            && !dialogue.visible()
            && !cutscenes.running();
        if hovered != clickable_item.last_hover {
            clickable_item.last_hover = hovered;
            if hovered {
                for mut sound in sound_query.p0().iter_mut() {
                    sound.play();
                }
            }
        }
        if hovered && clickable.just_clicked() {
            for mut sound in sound_query.p1().iter_mut() {
                sound.play();
            }
        }
        if hovered {
            if let Some(text) = text.as_mut() {
                text.sections[0].style.color = Color::WHITE;
            } else {
                *visibility = Visibility::Inherited;
            }
            if clickable.confirmed {
                input.reset(MouseButton::Left);
                match clickable_item.action {
                    ClickableAction::Tavern => {
                        ev_barkeep_quest.send_default();
                        input.reset(MouseButton::Left);
                    }
                    ClickableAction::Mayor => {
                        ev_mayor_quest.send_default();
                        input.reset(MouseButton::Left);
                    }
                    ClickableAction::ConcertHall => {
                        screen_fade.fade_out(0.5);
                        state.leave = OutsideLeave::LeaveToConcertHall;
                    }
                    ClickableAction::Leave => {
                        if game_state.quests.must_talk_to_mayor() {
                            for (p, t) in MUST_TALK_TO_MAYOR.iter() {
                                dialogue.add_text(*p, String::from(*t));
                            }
                        } else if !game_state.quests.talked_to_barkeep
                            && game_state.health != game_state.health_max
                        {
                            for (p, t) in MUST_TALK_TO_BARKEEP.iter() {
                                dialogue.add_text(*p, String::from(*t));
                            }
                        } else {
                            screen_fade.fade_out(0.5);
                            state.leave = OutsideLeave::LeaveToOverworld;
                            for mut source in sound_query.p2().iter_mut() {
                                source.stop();
                            }
                        }
                    }
                }
            }
        }
    }
}

fn outside_leave(
    state: Res<OutsideState>,
    screen_fade: Res<ScreenFade>,
    mut app_state: ResMut<NextState<AppState>>,
    mut game_state: ResMut<GameState>,
) {
    if screen_fade.faded_out() {
        if matches!(state.leave, OutsideLeave::LeaveToOverworld) {
            game_state.checkpoint();
            app_state.set(AppState::Overworld);
        } else if matches!(state.leave, OutsideLeave::LeaveToConcertHall) {
            app_state.set(AppState::TownConcertHall);
        }
    }
}

fn outside_pulsing_icons(mut query: Query<(&mut Transform2, &PulsingIcon)>, time: Res<Time>) {
    for (mut transform, icon) in query.iter_mut() {
        transform.scale = icon.scale + (Vec2::ONE * 0.05 * (time.elapsed_seconds() * 2.).sin());
    }
}

fn outside_tavern_icon(
    mut query: Query<&mut Visibility, With<TavernIcon>>,
    game_state: Res<GameState>,
) {
    for mut visibility in query.iter_mut() {
        *visibility = if game_state.health != game_state.health_max {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
}
fn outside_mayor_icon(
    mut query: Query<&mut Visibility, With<MayorIcon>>,
    game_state: Res<GameState>,
) {
    for mut visibility in query.iter_mut() {
        *visibility = if game_state.quests.must_talk_to_mayor() {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
}
fn outside_concert_hall_icon(
    mut query: Query<&mut Visibility, With<ConcertHallIcon>>,
    game_state: Res<GameState>,
) {
    for mut visibility in query.iter_mut() {
        *visibility = if game_state.skill_points > 0 && !game_state.has_all_unlocks() {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
}

pub mod rum_refill;
