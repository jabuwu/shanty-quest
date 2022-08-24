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
    entries: VecDeque<DialogueEntry>,
    time: f32,
    last_characters: usize,
}

#[derive(Clone)]
struct DialogueEntry {
    portrait: DialoguePortrait,
    text: String,
}

#[derive(Clone, PartialEq, Eq)]
pub enum DialoguePortrait {
    None,
    Jagerossa,
}

impl DialoguePortrait {
    fn name(&self) -> &'static str {
        match *self {
            Self::None => "???",
            Self::Jagerossa => "Captain Mick Jagerossa",
        }
    }
}

impl Dialogue {
    pub fn add_text(&mut self, portrait: DialoguePortrait, text: String) {
        self.entries.push_back(DialogueEntry { portrait, text });
    }

    pub fn clear(&mut self) {
        self.entries = VecDeque::new();
    }

    pub fn visible(&self) -> bool {
        self.entries.len() > 0
    }

    pub fn characters(&self) -> usize {
        if self.entries.len() > 0 {
            ((self.time * 50.) as usize).clamp(0, self.entries[0].text.len())
        } else {
            0
        }
    }

    pub fn all_characters_visible(&self) -> bool {
        if self.entries.len() > 0 {
            self.characters() == self.entries[0].text.len()
        } else {
            false
        }
    }
}

#[derive(Component)]
pub struct DialogueBack;

#[derive(Component)]
pub struct DialogueText;

#[derive(Component)]
pub struct DialogueName;

#[derive(Component)]
pub struct DialoguePortraitComp;

fn dialogue_init(
    mut ev_dialogue_init: EventReader<DialogueInitEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for _ in ev_dialogue_init.iter() {
        commands
            .spawn_bundle(Transform2Bundle {
                transform2: Transform2::from_xy(0., -240.),
                ..Default::default()
            })
            .insert_bundle(VisibilityBundle::default())
            .insert(FollowCamera {
                offset: Vec2::new(0., -240.),
            })
            .insert(Persistent)
            .with_children(|parent| {
                parent
                    .spawn_bundle(SpriteBundle {
                        texture: asset_library.sprite_dialogue_bg.clone(),
                        ..Default::default()
                    })
                    .insert(
                        Transform2::new()
                            .with_scale(Vec2::ONE * 1.4)
                            .with_depth(DEPTH_LAYER_DIALOGUE_BACK),
                    )
                    .insert(DialogueBack)
                    .insert(AudioPlusSource::new(
                        asset_library.sound_effects.sfx_dialogue_proceed.clone(),
                    ));
                parent
                    .spawn_bundle(Text2dBundle {
                        text: Text::from_section(
                            "",
                            TextStyle {
                                font: asset_library.font_default.clone(),
                                font_size: 48.0,
                                color: Color::rgb_u8(66, 53, 24),
                            },
                        )
                        .with_alignment(TextAlignment {
                            horizontal: HorizontalAlign::Left,
                            vertical: VerticalAlign::Top,
                        }),
                        ..Default::default()
                    })
                    .insert(Transform2::from_xy(-540., 60.).with_depth(DEPTH_LAYER_DIALOGUE_TEXT))
                    .insert(DialogueText)
                    .insert(AudioPlusSource::new(
                        asset_library.sound_effects.sfx_dialogue_start.clone(),
                    ));
                parent
                    .spawn_bundle(Text2dBundle {
                        text: Text::from_section(
                            "",
                            TextStyle {
                                font: asset_library.font_default.clone(),
                                font_size: 32.0,
                                color: Color::WHITE,
                            },
                        )
                        .with_alignment(TextAlignment {
                            horizontal: HorizontalAlign::Left,
                            vertical: VerticalAlign::Center,
                        }),
                        ..Default::default()
                    })
                    .insert(Transform2::from_xy(-550., 90.).with_depth(DEPTH_LAYER_DIALOGUE_TEXT))
                    .insert(DialogueName);
                parent
                    .spawn_bundle(SpriteBundle {
                        texture: asset_library.sprite_dialogue_portrait_guitar.clone(),
                        ..Default::default()
                    })
                    .insert(
                        Transform2::from_xy(350., 280.).with_depth(DEPTH_LAYER_DIALOGUE_PORTRAIT),
                    )
                    .insert(DialoguePortraitComp)
                    .insert(AudioPlusSource::new(
                        asset_library.sound_effects.sfx_dialogue_repeat.clone(),
                    ));
            });
    }
}

fn dialogue_update(
    mut dialogue: ResMut<Dialogue>,
    mut queries: ParamSet<(
        Query<&mut Visibility, With<DialogueBack>>,
        Query<&mut Text, With<DialogueText>>,
        Query<&mut Text, With<DialogueName>>,
        Query<&mut Visibility, With<DialoguePortraitComp>>,
        Query<&mut AudioPlusSource, With<DialogueBack>>,
        Query<&mut AudioPlusSource, With<DialoguePortraitComp>>,
        Query<&mut AudioPlusSource, With<DialogueText>>,
    )>,
    screen_fade: Res<ScreenFade>,
    mut input: ResMut<Input<KeyCode>>,
    mut mouse: ResMut<Input<MouseButton>>,
    time: Res<Time>,
) {
    let allow = screen_fade.faded_in();
    if (input.just_pressed(KeyCode::Space) || mouse.just_pressed(MouseButton::Left)) && allow {
        if !dialogue.entries.is_empty() {
            if dialogue.all_characters_visible() {
                dialogue.entries.pop_front();
                dialogue.time = 0.;
                input.reset(KeyCode::Space);
                mouse.reset(MouseButton::Left);
                for mut sound in queries.p4().iter_mut() {
                    sound.play();
                }
            } else {
                dialogue.time = 999999.;
            }
        }
    }
    let mut hide = false;
    if let Some(entry) = dialogue.entries.get(0).cloned() {
        if allow {
            let characters = dialogue.characters();
            let characters_sfx = characters / 7;
            if dialogue.last_characters != characters_sfx {
                if dialogue.last_characters == 0 {
                    for mut sound in queries.p6().iter_mut() {
                        sound.play();
                    }
                } else {
                    for mut sound in queries.p5().iter_mut() {
                        sound.play();
                    }
                }
                dialogue.last_characters = characters_sfx;
            }
            dialogue.time += time.delta_seconds();
            for mut back_visibility in queries.p0().iter_mut() {
                back_visibility.is_visible = true;
            }
            for mut dialogue_text in queries.p1().iter_mut() {
                dialogue_text.sections[0].value = String::from(&entry.text[0..characters]);
            }
            for mut dialogue_name in queries.p2().iter_mut() {
                dialogue_name.sections[0].value = String::from(entry.portrait.name());
            }
            for mut portrait_visibility in queries.p3().iter_mut() {
                portrait_visibility.is_visible = entry.portrait == DialoguePortrait::Jagerossa;
            }
        } else {
            hide = true;
        }
    } else {
        hide = true;
    }
    if hide {
        for mut back_visibility in queries.p0().iter_mut() {
            back_visibility.is_visible = false;
        }
        for mut dialogue_text in queries.p1().iter_mut() {
            if dialogue_text.sections[0].value != "" {
                dialogue_text.sections[0].value = "".to_owned();
            }
        }
        for mut dialogue_name in queries.p2().iter_mut() {
            if dialogue_name.sections[0].value != "" {
                dialogue_name.sections[0].value = "".to_owned();
            }
        }
        for mut portrait_visibility in queries.p3().iter_mut() {
            portrait_visibility.is_visible = false;
        }
    }
}
