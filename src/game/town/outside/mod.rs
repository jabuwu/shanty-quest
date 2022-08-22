use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;

pub struct OutsidePlugin;

impl Plugin for OutsidePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::TownOutside).with_system(outside_init))
            .add_system(outside_click);
    }
}

#[derive(Component)]
struct ClickableItem {
    click_priority: i32,
    action: ClickableAction,
}

#[derive(PartialEq, Eq, Debug)]
enum ClickableAction {
    Tavern,
    Mayor,
    ConcertHall,
    Leave,
}

fn outside_init(
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
    game_state: Res<GameState>,
) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_library.sprite_town_bg.clone(),
            ..Default::default()
        })
        .insert(
            Transform2::new()
                .with_depth((DepthLayer::Front, 0.))
                .with_scale(Vec2::ONE * 0.5),
        );
    commands
        .spawn_bundle(VisibilityBundle {
            visibility: Visibility { is_visible: false },
            ..Default::default()
        })
        .insert_bundle(TransformBundle::default())
        .insert(Transform2::from_xy(-493., -84.).with_depth((DepthLayer::Front, 0.1)))
        .insert(ClickableItem {
            click_priority: 1,
            action: ClickableAction::Tavern,
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
                .insert(Transform2::from_xy(0., 30.).with_depth((DepthLayer::Front, 0.1)));
        });
    commands
        .spawn_bundle(VisibilityBundle {
            visibility: Visibility { is_visible: false },
            ..Default::default()
        })
        .insert_bundle(TransformBundle::default())
        .insert(Transform2::from_xy(369., 0.).with_depth((DepthLayer::Front, 0.1)))
        .insert(ClickableItem {
            click_priority: 0,
            action: ClickableAction::Mayor,
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
                .insert(Transform2::from_xy(0., 43.).with_depth((DepthLayer::Front, 0.1)));
        });
    commands
        .spawn_bundle(VisibilityBundle {
            visibility: Visibility { is_visible: false },
            ..Default::default()
        })
        .insert_bundle(TransformBundle::default())
        .insert(Transform2::from_xy(-229., 72.).with_depth((DepthLayer::Front, 0.1)))
        .insert(ClickableItem {
            click_priority: 0,
            action: ClickableAction::ConcertHall,
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
                .insert(Transform2::from_xy(115., -5.).with_depth((DepthLayer::Front, 0.1)));
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
        })
        .insert(Transform2::from_xy(470., -330.).with_depth((DepthLayer::Front, 0.2)));
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
        .insert(Transform2::from_xy(0., 330.).with_depth((DepthLayer::Front, 0.2)));
}

fn outside_click(
    mut query: Query<(
        &mut Visibility,
        &Clickable,
        &ClickableItem,
        Option<&mut Text>,
    )>,
    mut app_state: ResMut<State<AppState>>,
    mut input: ResMut<Input<MouseButton>>,
) {
    let mut highest_priority = -1;
    for (_, clickable, clickable_item, _) in query.iter_mut() {
        if clickable.hovered && clickable_item.click_priority > highest_priority {
            highest_priority = clickable_item.click_priority;
        }
    }
    for (mut visibility, clickable, clickable_item, mut text) in query.iter_mut() {
        if let Some(text) = text.as_mut() {
            text.sections[0].style.color = Color::BLACK;
        } else {
            visibility.is_visible = false;
        }
        if clickable.hovered && highest_priority == clickable_item.click_priority {
            if let Some(text) = text.as_mut() {
                text.sections[0].style.color = Color::WHITE;
            } else {
                visibility.is_visible = true;
            }
            if clickable.confirmed {
                input.reset(MouseButton::Left);
                match clickable_item.action {
                    ClickableAction::Tavern => {
                        app_state.set(AppState::TownTavern).unwrap();
                    }
                    ClickableAction::Mayor => {
                        app_state.set(AppState::TownMayor).unwrap();
                    }
                    ClickableAction::ConcertHall => {
                        app_state.set(AppState::TownConcertHall).unwrap();
                    }
                    ClickableAction::Leave => {
                        app_state.set(AppState::Overworld).unwrap();
                    }
                }
            }
        }
    }
}
