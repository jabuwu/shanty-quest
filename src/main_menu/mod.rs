use crate::common::prelude::*;
use bevy::{app::AppExit, prelude::*};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(setup))
            .add_system_set(SystemSet::on_update(GameState::MainMenu).with_system(button_system));
    }
}

#[derive(Component)]
pub struct StartGameButton;

#[derive(Component)]
pub struct QuitButton;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut UiColor,
            Option<&StartGameButton>,
            Option<&QuitButton>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut game_state: ResMut<State<GameState>>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, mut color, start_game, quit) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                if color.0 == PRESSED_BUTTON {
                    if start_game.is_some() {
                        game_state.set(GameState::Game).unwrap();
                    }
                    if quit.is_some() {
                        exit.send(AppExit);
                    }
                }
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn setup(mut commands: Commands, asset_library: Res<AssetLibrary>) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::ColumnReverse,
                        size: Size::new(Val::Percent(30.0), Val::Percent(30.0)),
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    color: Color::NONE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Px(250.0), Val::Px(65.0)),
                                margin: UiRect::all(Val::Auto),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            color: NORMAL_BUTTON.into(),
                            ..default()
                        })
                        .insert(StartGameButton)
                        .with_children(|parent| {
                            parent.spawn_bundle(TextBundle::from_section(
                                "Start Game",
                                TextStyle {
                                    font: asset_library.font_default.clone(),
                                    font_size: 40.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            ));
                        });
                    parent
                        .spawn_bundle(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Px(250.0), Val::Px(65.0)),
                                margin: UiRect::all(Val::Auto),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            color: NORMAL_BUTTON.into(),
                            ..default()
                        })
                        .insert(QuitButton)
                        .with_children(|parent| {
                            parent.spawn_bundle(TextBundle::from_section(
                                "Quit",
                                TextStyle {
                                    font: asset_library.font_default.clone(),
                                    font_size: 40.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            ));
                        });
                });
        });
}
