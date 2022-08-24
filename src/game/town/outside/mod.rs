use crate::common::prelude::*;
use crate::game::prelude::*;
use audio_plus::prelude::*;
use bevy::prelude::*;

#[derive(Default)]
pub struct OutsideState {
    leave: bool,
}

pub struct OutsidePlugin;

impl Plugin for OutsidePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<OutsideState>()
            .add_system_set(SystemSet::on_enter(AppState::TownOutside).with_system(outside_init))
            .add_system_set(SystemSet::on_update(AppState::TownOutside).with_system(outside_leave))
            .add_system(outside_click);
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

fn outside_init(
    mut state: ResMut<OutsideState>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
    game_state: Res<GameState>,
) {
    *state = OutsideState::default();
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(Transform2::new().with_depth((DepthLayer::Camera, 0.)));
    commands
        .spawn()
        .insert(AudioPlusSource::new(
            asset_library.sound_effects.sfx_town_outside_hover.clone(),
        ))
        .insert(HoverSound);
    commands
        .spawn()
        .insert(AudioPlusSource::new(
            asset_library.sound_effects.sfx_town_outside_click.clone(),
        ))
        .insert(ClickSound);
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_library.sprite_town_bg.clone(),
            ..Default::default()
        })
        .insert(
            Transform2::new()
                .with_depth(DEPTH_LAYER_TOWN_OUTSIDE_BG)
                .with_scale(Vec2::ONE * 0.5),
        );
    commands
        .spawn_bundle(VisibilityBundle {
            visibility: Visibility { is_visible: false },
            ..Default::default()
        })
        .insert_bundle(TransformBundle::default())
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
                .spawn_bundle(SpriteBundle {
                    texture: asset_library.sprite_town_tavern_outline.clone(),
                    ..Default::default()
                })
                .insert(
                    Transform2::from_xy(0., 30.).with_depth(DEPTH_LAYER_TOWN_OUTSIDE_HIGHLIGHT),
                );
        });
    commands
        .spawn_bundle(VisibilityBundle {
            visibility: Visibility { is_visible: false },
            ..Default::default()
        })
        .insert_bundle(TransformBundle::default())
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
                .spawn_bundle(SpriteBundle {
                    texture: asset_library.sprite_town_mayor_outline.clone(),
                    ..Default::default()
                })
                .insert(
                    Transform2::from_xy(0., 43.).with_depth(DEPTH_LAYER_TOWN_OUTSIDE_HIGHLIGHT),
                );
        });
    commands
        .spawn_bundle(VisibilityBundle {
            visibility: Visibility { is_visible: false },
            ..Default::default()
        })
        .insert_bundle(TransformBundle::default())
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
                .spawn_bundle(SpriteBundle {
                    texture: asset_library.sprite_town_concert_hall_outline.clone(),
                    ..Default::default()
                })
                .insert(
                    Transform2::from_xy(115., -5.).with_depth(DEPTH_LAYER_TOWN_OUTSIDE_HIGHLIGHT),
                );
        });
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section(
                "Exit Town".to_owned(),
                TextStyle {
                    font: asset_library.font_default.clone(),
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
        .insert(ClickableItem {
            click_priority: 0,
            action: ClickableAction::Leave,
            last_hover: false,
        })
        .insert(Transform2::from_xy(470., -330.).with_depth(DEPTH_LAYER_TOWN_OUTSIDE_EXIT));
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section(
                game_state.town.name.clone(),
                TextStyle {
                    font: asset_library.font_default.clone(),
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
        .insert(Transform2::from_xy(0., 330.).with_depth(DEPTH_LAYER_TOWN_OUTSIDE_NAME));
}

fn outside_click(
    mut query: Query<(
        &mut Visibility,
        &Clickable,
        &mut ClickableItem,
        Option<&mut Text>,
    )>,
    mut app_state: ResMut<State<AppState>>,
    mut input: ResMut<Input<MouseButton>>,
    mut screen_fade: ResMut<ScreenFade>,
    mut state: ResMut<OutsideState>,
    mut sound_query: ParamSet<(
        Query<&mut AudioPlusSource, With<HoverSound>>,
        Query<&mut AudioPlusSource, With<ClickSound>>,
    )>,
    state_time: Res<StateTime<AppState>>,
    mut dialogue: ResMut<Dialogue>,
    cutscenes: Res<Cutscenes>,
    game_state: Res<GameState>,
    mut ev_mayor_quest: EventWriter<QuestMayorEvent>,
    mut ev_barkeep_quest: EventWriter<QuestBarkeepEvent>,
) {
    if state_time.just_entered() || state.leave {
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
            visibility.is_visible = false;
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
                visibility.is_visible = true;
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
                        app_state.set(AppState::TownConcertHall).unwrap();
                    }
                    ClickableAction::Leave => {
                        if game_state.quests.must_talk_to_mayor() {
                            for (p, t) in MUST_TALK_TO_MAYOR.iter() {
                                dialogue.add_text(*p, String::from(*t));
                            }
                        } else {
                            screen_fade.fade_out(1.);
                            state.leave = true;
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
    mut app_state: ResMut<State<AppState>>,
) {
    if state.leave && screen_fade.faded_out() {
        app_state.set(AppState::Overworld).unwrap();
    }
}
