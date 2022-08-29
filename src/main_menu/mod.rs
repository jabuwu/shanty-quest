use crate::{common::prelude::*, game::state::GameState};
use audio_plus::prelude::*;
use bevy::prelude::*;

use self::slider::VolumeSliderSpawnEvent;

const LOGO_POSITION: Vec2 = Vec2::new(0., 115.);
const LOGO_SCALE: Vec2 = Vec2::new(0.84, 0.84);
const LOGO_MOVEMENT_GROW: Vec2 = Vec2::new(1., 1.4);
const BUTTON_SCALE: Vec2 = Vec2::new(0.72, 0.72);
const BUTTON_POSITION: Vec2 = Vec2::new(80., -200.);
const BUTTON_TEXT_SCALE: Vec2 = Vec2::new(0.8, 0.8);

#[derive(Default)]
struct MenuState {
    play: bool,
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(slider::VolumeSliderPlugin)
            .init_resource::<MenuState>()
            .add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(menu_setup))
            .add_system_set(SystemSet::on_update(AppState::MainMenu).with_system(menu_fade))
            .add_system(menu_logo)
            .add_system(menu_shine)
            .add_system(menu_button)
            .add_system(menu_background_move);
    }
}

#[derive(Component)]
struct Button {
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

#[derive(Component)]
struct Sound;

#[derive(Component)]
struct Logo {
    x: f32,
}

#[derive(Component)]
struct Shine {
    x: f32,
}

#[derive(Component)]
struct Background;

fn menu_setup(
    mut menu_state: ResMut<MenuState>,
    mut screen_fade: ResMut<ScreenFade>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
    mut cutscenes: ResMut<Cutscenes>,
    mut dialogue: ResMut<Dialogue>,
    mut ev_volume_slider_spawn: EventWriter<VolumeSliderSpawnEvent>,
) {
    *menu_state = MenuState::default();
    cutscenes.clear();
    dialogue.clear();
    screen_fade.fade_in(1.);
    ev_volume_slider_spawn.send_default();
    commands.spawn_bundle(Camera2dBundle::default());
    commands
        .spawn()
        .insert(
            AudioPlusSource::new(asset_library.sound_effects.sfx_menu_ambient.clone()).as_looping(),
        )
        .insert(Sound);
    commands
        .spawn()
        .insert(
            AudioPlusSource::new(asset_library.sound_effects.sfx_menu_music.clone()).as_looping(),
        )
        .insert(Sound);
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
            texture: asset_library.menu_sprite_back.clone(),
            ..Default::default()
        })
        .insert(
            Transform2::new()
                .with_scale(Vec2::ONE * 0.73)
                .with_depth((DepthLayer::Front, 0.))
                .without_pixel_perfect(),
        )
        .insert(Background);
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(1., 1., 1., 0.),
                ..Default::default()
            },
            texture: asset_library.menu_sprite_logo.clone(),
            ..Default::default()
        })
        .insert(Transform2::from_xy(0., 90.).with_depth((DepthLayer::Front, 0.2)))
        .insert(Label("Logo".to_owned()))
        .insert(Logo { x: 0. });
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(1., 1., 1., 0.),
                ..Default::default()
            },
            texture: asset_library.menu_sprite_shine.clone(),
            ..Default::default()
        })
        .insert(
            Transform2::from_xy(0., 90.)
                .with_scale(LOGO_SCALE * 0.8)
                .with_depth((DepthLayer::Front, 0.1)),
        )
        .insert(Shine { x: 0. })
        .insert(Label("Shine".to_owned()));
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_library.menu_sprite_button_back.clone(),
            ..Default::default()
        })
        .insert(Button {
            shape: CollisionShape::Rect {
                size: Vec2::new(406., 159.) * BUTTON_SCALE,
            },
            last_hover: false,
            clicked: false,
            audio_hover: hover_audio,
            audio_click: click_audio,
            audio_click_confirm: click_confirm_audio,
        })
        .insert(
            Transform2::from_translation(BUTTON_POSITION)
                .with_scale(Vec2::ONE * BUTTON_SCALE)
                .with_depth((DepthLayer::Front, 0.3)),
        )
        .insert(Label("Play Button".to_owned()))
        .with_children(|parent| {
            parent
                .spawn_bundle(SpriteBundle {
                    texture: asset_library.menu_sprite_button_play_normal.clone(),
                    ..Default::default()
                })
                .insert(
                    Transform2::new()
                        .with_scale(BUTTON_TEXT_SCALE)
                        .with_depth((DepthLayer::Front, 0.4)),
                )
                .insert(ButtonText {
                    normal: asset_library.menu_sprite_button_play_normal.clone(),
                    hover: asset_library.menu_sprite_button_play_hover.clone(),
                    press: asset_library.menu_sprite_button_play_press.clone(),
                });
            parent
                .spawn_bundle(SpriteBundle {
                    texture: asset_library.menu_sprite_skull.clone(),
                    ..Default::default()
                })
                .insert(
                    Transform2::from_xy(-210., 12.)
                        .with_scale(Vec2::ONE * 1.2)
                        .with_depth((DepthLayer::Front, 0.4)),
                );
        });

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_library.menu_fullscreen_recommended.clone(),
            ..Default::default()
        })
        .insert(
            Transform2::from_xy(528., -305.)
                .with_depth((DepthLayer::Front, 0.2))
                .with_scale(Vec2::ONE * 0.5),
        );
}

fn menu_logo(mut query: Query<(&mut Logo, &mut Transform2, &mut Sprite)>, time: Res<Time>) {
    for (mut logo, mut transform, mut sprite) in query.iter_mut() {
        logo.x += time.delta_seconds() * 3.;
        logo.x = logo.x.clamp(0., 1.);
        transform.translation =
            Vec2::new(0., 300.).lerp(LOGO_POSITION, ease(Easing::BackOut, logo.x));
        transform.scale =
            (LOGO_SCALE * LOGO_MOVEMENT_GROW).lerp(LOGO_SCALE, ease(Easing::BackOut, logo.x));
        sprite.color.set_a(ease(Easing::QuartOut, logo.x));
    }
}

fn menu_shine(mut query: Query<(&mut Shine, &mut Transform2, &mut Sprite)>, time: Res<Time>) {
    for (mut shine, mut transform, mut sprite) in query.iter_mut() {
        shine.x += time.delta_seconds();
        sprite
            .color
            .set_a(ease(Easing::QuartOut, (shine.x * 3.).clamp(0., 1.)));
        transform.rotation = shine.x * 0.2;
        transform.scale = Vec2::new(shine.x.sin() * 0.2 + 0.6, shine.x.cos() * 0.2 + 0.6)
    }
}

fn play_sound(entity: Entity, sfx_query: &mut Query<&mut AudioPlusSource>) {
    if let Ok(mut source) = sfx_query.get_mut(entity) {
        source.play();
    }
}

fn menu_button(
    mut screen_fade: ResMut<ScreenFade>,
    mut button_query: Query<(&mut Button, &GlobalTransform, &Children, &mut Transform2)>,
    mut text_query: Query<(&ButtonText, &mut Handle<Image>)>,
    mut sfx_query: Query<&mut AudioPlusSource>,
    sound_query: Query<Entity, With<Sound>>,
    mouse: Res<Mouse>,
    input: Res<Input<MouseButton>>,
    mut menu_state: ResMut<MenuState>,
) {
    for (mut button, transform, children, mut transform2) in button_query.iter_mut() {
        let hover = !menu_state.play
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
                for entity in sound_query.iter() {
                    if let Ok(mut source) = sfx_query.get_mut(entity) {
                        source.stop();
                    }
                }
                menu_state.play = true;
                play_sound(button.audio_click_confirm, &mut sfx_query);
                screen_fade.fade_out(1.8);
            }
            button.clicked = false;
        }
        transform2.translation = BUTTON_POSITION;
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
    menu_state: Res<MenuState>,
    mut game_state: ResMut<GameState>,
    mut app_state: ResMut<State<AppState>>,
    screen_fade: Res<ScreenFade>,
) {
    if menu_state.play && screen_fade.faded_out() {
        *game_state = GameState::default();
        app_state.set(AppState::IntroCutscene).unwrap();
    }
}

fn menu_background_move(mut query: Query<&mut Transform2, With<Background>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        let time = time.time_since_startup().as_secs_f32();
        let time_x = (time * 0.1) % 2.;
        let time_y = (time * 0.12) % 2.;
        let baf_x = if time_x < 1. { time_x } else { 2.0 - time_x };
        let baf_y = if time_y < 1. { time_y } else { 2.0 - time_y };
        let x = ease(Easing::BackInOut, baf_x) * 10. - 5.;
        let y = ease(Easing::BackInOut, baf_y) * 10. - 5.;
        transform.translation = Vec2::new(x, y);
    }
}

pub mod slider;
