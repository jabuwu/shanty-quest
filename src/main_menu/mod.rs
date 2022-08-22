use crate::{common::prelude::*, game::state::GameState};
use audio_plus::prelude::*;
use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(menu_setup))
            .add_system(menu_button);
    }
}

#[derive(Component)]
struct Button {
    shape: CollisionShape,
    clicked: bool,
}

#[derive(Component)]
struct ButtonText {
    normal: Handle<Image>,
    hover: Handle<Image>,
    press: Handle<Image>,
}

fn menu_setup(mut commands: Commands, asset_library: Res<AssetLibrary>) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn().insert(
        AudioPlusSource::new(asset_library.sound_effects.sfx_menu_ambient.clone()).as_looping(),
    );
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_library.menu_back.clone(),
            ..Default::default()
        })
        .insert(
            Transform2::new()
                .with_scale(Vec2::ONE * 0.71)
                .with_depth((DepthLayer::Front, 0.)),
        );
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_library.menu_logo.clone(),
            ..Default::default()
        })
        .insert(Transform2::from_xy(0., 90.).with_depth((DepthLayer::Front, 0.)))
        .insert(Label("Logo".to_owned()));
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_library.menu_button_back.clone(),
            ..Default::default()
        })
        .insert(Button {
            shape: CollisionShape::Rect {
                size: Vec2::new(406., 159.),
            },
            clicked: false,
        })
        .insert(Transform2::from_xy(0., -280.).with_depth((DepthLayer::Front, 0.1)))
        .insert(Label("Play Button".to_owned()))
        .with_children(|parent| {
            parent
                .spawn_bundle(SpriteBundle {
                    texture: asset_library.menu_button_play_normal.clone(),
                    ..Default::default()
                })
                .insert(Transform2::new().with_depth((DepthLayer::Front, 0.2)))
                .insert(ButtonText {
                    normal: asset_library.menu_button_play_normal.clone(),
                    hover: asset_library.menu_button_play_hover.clone(),
                    press: asset_library.menu_button_play_press.clone(),
                });
        });
}

fn menu_button(
    mut button_query: Query<(&mut Button, &GlobalTransform, &Children, &mut Transform2)>,
    mut text_query: Query<(&ButtonText, &mut Handle<Image>)>,
    mouse: Res<Mouse>,
    input: Res<Input<MouseButton>>,
    mut game_state: ResMut<GameState>,
    mut app_state: ResMut<State<AppState>>,
) {
    for (mut button, transform, children, mut transform2) in button_query.iter_mut() {
        let hover = button.shape.overlaps(
            transform.translation().truncate(),
            CollisionShape::Point,
            mouse.position,
        );
        if hover && input.just_pressed(MouseButton::Left) {
            button.clicked = true;
        }
        if button.clicked && input.just_released(MouseButton::Left) {
            if hover {
                *game_state = GameState::default();
                app_state.set(AppState::GameOverworld).unwrap();
            }
            button.clicked = false;
        }
        transform2.translation = Vec2::new(0., -280.);
        if button.clicked && hover {
            transform2.translation += Vec2::new(-2., -2.);
        }
        for child in children.iter() {
            if let Ok((text, mut image)) = text_query.get_mut(*child) {
                if button.clicked && hover {
                    *image = text.press.clone();
                } else if hover {
                    *image = text.hover.clone();
                } else {
                    *image = text.normal.clone();
                }
            }
        }
    }
}
