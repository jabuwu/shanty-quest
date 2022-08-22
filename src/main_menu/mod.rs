use crate::{common::prelude::*, game::state::GameState};
use audio_plus::prelude::*;
use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(menu_setup))
            .add_system(menu_button)
            .add_system(menu_fade);
    }
}

#[derive(Component)]
struct Fade {
    opacity: f32,
    out: bool,
}

#[derive(Component)]
struct Button {
    disabled: bool,
    shape: CollisionShape,
    last_hover: bool,
    clicked: bool,
    audio_hover: Entity,
    audio_click: Entity,
    audio_click_confirm: Entity,
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
    let hover_audio = commands
        .spawn()
        .insert(AudioPlusSource::new(
            asset_library.sound_effects.sfx_menu_button_hover.clone(),
        ))
        .id();
    let click_audio = commands
        .spawn()
        .insert(AudioPlusSource::new(
            asset_library.sound_effects.sfx_menu_button_click.clone(),
        ))
        .id();
    let click_confirm_audio = commands
        .spawn()
        .insert(AudioPlusSource::new(
            asset_library
                .sound_effects
                .sfx_menu_button_click_confirm
                .clone(),
        ))
        .id();
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::ONE * 50000.),
                color: Color::BLACK,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Transform2::new().with_depth((DepthLayer::Front, 1.)))
        .insert(Fade {
            opacity: 1.,
            out: false,
        });
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
            disabled: false,
            shape: CollisionShape::Rect {
                size: Vec2::new(406., 159.),
            },
            last_hover: false,
            clicked: false,
            audio_hover: hover_audio,
            audio_click: click_audio,
            audio_click_confirm: click_confirm_audio,
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

fn play_sound(entity: Entity, sfx_query: &mut Query<&mut AudioPlusSource>) {
    if let Ok(mut source) = sfx_query.get_mut(entity) {
        source.play();
    }
}

fn menu_button(
    mut button_query: Query<(&mut Button, &GlobalTransform, &Children, &mut Transform2)>,
    mut text_query: Query<(&ButtonText, &mut Handle<Image>)>,
    mut sfx_query: Query<&mut AudioPlusSource>,
    mut fade_query: Query<&mut Fade>,
    mouse: Res<Mouse>,
    input: Res<Input<MouseButton>>,
) {
    for (mut button, transform, children, mut transform2) in button_query.iter_mut() {
        let hover = !button.disabled
            && button.shape.overlaps(
                transform.translation().truncate(),
                CollisionShape::Point,
                mouse.position,
            );
        if hover != button.last_hover {
            if hover && !button.clicked {
                play_sound(button.audio_hover, &mut sfx_query);
            }
            button.last_hover = hover;
        }
        if hover && input.just_pressed(MouseButton::Left) {
            button.clicked = true;
            play_sound(button.audio_click, &mut sfx_query);
        }
        if button.clicked && input.just_released(MouseButton::Left) {
            if hover {
                button.disabled = true;
                play_sound(button.audio_click_confirm, &mut sfx_query);
                if let Ok(mut fade) = fade_query.get_single_mut() {
                    fade.out = true;
                }
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

fn menu_fade(
    mut query: Query<(&mut Fade, &mut Sprite)>,
    time: Res<Time>,
    mut game_state: ResMut<GameState>,
    mut app_state: ResMut<State<AppState>>,
) {
    for (mut fade, mut sprite) in query.iter_mut() {
        if fade.out {
            fade.opacity += time.delta_seconds();
        } else {
            fade.opacity -= time.delta_seconds() * 2.;
        }
        fade.opacity = fade.opacity.clamp(0., 1.);
        sprite.color.set_a(fade.opacity);
        if fade.out && fade.opacity == 1. {
            *game_state = GameState::default();
            app_state.set(AppState::GameOverworld).unwrap();
        }
    }
}
