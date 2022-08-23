use crate::common::prelude::*;
use audio_plus::prelude::*;
use bevy::prelude::*;
use std::collections::VecDeque;

pub struct DialoguePlugin;

impl Plugin for DialoguePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Dialogue>()
            .add_event::<DialogueInitEvent>()
            .add_system(dialogue_init)
            .add_system(dialogue_update);
    }
}

#[derive(Default, Clone, Copy)]
pub struct DialogueInitEvent;

#[derive(Default)]
pub struct Dialogue {
    texts: VecDeque<String>,
}

impl Dialogue {
    pub fn add_text(&mut self, text: String) {
        self.texts.push_back(text);
    }

    pub fn clear(&mut self) {
        self.texts = VecDeque::new();
    }

    pub fn visible(&self) -> bool {
        self.texts.len() > 0
    }
}

#[derive(Component)]
pub struct DialogueBack;

#[derive(Component)]
pub struct DialogueText;

fn dialogue_init(
    mut ev_dialogue_init: EventReader<DialogueInitEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for _ in ev_dialogue_init.iter() {
        commands
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(40.0)),
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        bottom: Val::Px(0.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                visibility: Visibility { is_visible: false },
                color: Color::rgba(0., 0., 0., 0.97).into(),
                ..Default::default()
            })
            .insert(AudioPlusSource::new(
                asset_library.sound_effects.sfx_dialogue_progress.clone(),
            ))
            .insert(DialogueBack)
            .insert(Persistent)
            .with_children(|parent| {
                parent
                    .spawn_bundle(TextBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            position: UiRect {
                                top: Val::Px(20.),
                                left: Val::Px(20.),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        text: Text::from_section(
                            "",
                            TextStyle {
                                font: asset_library.font_default.clone(),
                                font_size: 32.0,
                                color: Color::rgba(1., 1., 1., 1.0),
                            },
                        )
                        .with_alignment(TextAlignment {
                            horizontal: HorizontalAlign::Left,
                            vertical: VerticalAlign::Top,
                        }),
                        ..Default::default()
                    })
                    .insert(DialogueText);
            });
    }
}

fn dialogue_update(
    mut dialogue: ResMut<Dialogue>,
    mut back_query: Query<&mut Visibility, With<DialogueBack>>,
    mut text_query: Query<&mut Text, With<DialogueText>>,
    mut sound_query: Query<&mut AudioPlusSource, With<DialogueBack>>,
    screen_fade: Res<ScreenFade>,
    input: Res<Input<KeyCode>>,
) {
    let allow = screen_fade.faded_in();
    if input.just_pressed(KeyCode::Space) && allow {
        if dialogue.texts.pop_front().is_some() {
            for mut sound in sound_query.iter_mut() {
                sound.play();
            }
        }
    }
    if let Some(text) = dialogue.texts.get(0) {
        if allow {
            for mut back_visibility in back_query.iter_mut() {
                back_visibility.is_visible = true;
            }
            for mut ui_text in text_query.iter_mut() {
                if ui_text.sections[0].value != *text {
                    ui_text.sections[0].value = text.clone();
                }
            }
        } else {
            for mut back_visibility in back_query.iter_mut() {
                back_visibility.is_visible = false;
            }
            for mut text in text_query.iter_mut() {
                if text.sections[0].value != "" {
                    text.sections[0].value = "".to_owned();
                }
            }
        }
    } else {
        for mut back_visibility in back_query.iter_mut() {
            back_visibility.is_visible = false;
        }
        for mut text in text_query.iter_mut() {
            if text.sections[0].value != "" {
                text.sections[0].value = "".to_owned();
            }
        }
    }
}
