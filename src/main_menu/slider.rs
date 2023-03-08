use crate::common::{label::Label, prelude::*};
use audio_plus::prelude::*;
use bevy::{prelude::*, sprite::Anchor};

pub struct VolumeSliderPlugin;

impl Plugin for VolumeSliderPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<VolumeSliderSpawnEvent>()
            .add_system(volume_slider_spawn)
            .add_system(volume_slider_update);
    }
}

#[derive(Default, Clone, Copy)]
pub struct VolumeSliderSpawnEvent;

#[derive(Component)]
pub struct VolumeSliderKnob;

fn volume_slider_spawn(
    mut ev_spawn: EventReader<VolumeSliderSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for _ in ev_spawn.iter() {
        commands
            .spawn(VisibilityBundle::default())
            .insert(TransformBundle::default())
            .insert(FollowCamera { offset: Vec2::ZERO })
            .insert(Transform2::from_xy(41., -312.).without_pixel_perfect())
            .with_children(|parent| {
                parent
                    .spawn(SpriteBundle {
                        texture: asset_library.menu_slider_back.clone(),
                        ..Default::default()
                    })
                    .insert(
                        Transform2::from_xy(0., 0.)
                            .with_scale(Vec2::ONE * 0.5)
                            .with_depth((DepthLayer::Front, 0.5)),
                    );
                parent
                    .spawn(SpriteBundle {
                        texture: asset_library.menu_slider_knob.clone(),
                        ..Default::default()
                    })
                    .insert(
                        Transform2::from_xy(0., 0.)
                            .with_scale(Vec2::ONE * 0.5)
                            .with_depth((DepthLayer::Front, 0.51)),
                    )
                    .insert(VolumeSliderKnob)
                    .insert(Clickable {
                        shape: CollisionShape::Rect {
                            size: Vec2::new(100., 100.),
                        },
                        use_global: true,
                        ..Default::default()
                    });
                parent
                    .spawn(SpriteBundle {
                        texture: asset_library.menu_slider_icon.clone(),
                        ..Default::default()
                    })
                    .insert(
                        Transform2::from_xy(-100., 0.)
                            .with_scale(Vec2::ONE * 0.5)
                            .with_depth((DepthLayer::Front, 0.51)),
                    );
            });
        commands
            .spawn(Text2dBundle {
                text: Text::from_section(
                    "change volume anytime with O/P",
                    TextStyle {
                        font: asset_library.font_bold.clone(),
                        font_size: 24.0,
                        color: Color::BLACK,
                    },
                )
                .with_alignment(TextAlignment::Center),
                text_anchor: Anchor::Center,
                ..Default::default()
            })
            .insert(
                Transform2::from_translation(Vec2::new(40., -353.))
                    .with_depth((DepthLayer::Front, 0.51)),
            )
            .insert(Label("A".into()));
    }
}

fn volume_slider_update(
    mut query: Query<(&mut Transform2, &Clickable), With<VolumeSliderKnob>>,
    mouse: Res<Mouse>,
    mut mixer: ResMut<AudioPlusMixer>,
    asset_library: Res<AssetLibrary>,
    mut commands: Commands,
) {
    for (mut transform, clickable) in query.iter_mut() {
        if clickable.just_clicked() {
            commands
                .spawn_empty()
                .insert(
                    AudioPlusSource::new(asset_library.sound_effects.sfx_menu_button_click.clone())
                        .as_playing(),
                )
                .insert(TimeToLive { seconds: 3. });
        }
        if clickable.just_released() {
            commands
                .spawn_empty()
                .insert(
                    AudioPlusSource::new(asset_library.sound_effects.sfx_audio_preview.clone())
                        .as_playing(),
                )
                .insert(TimeToLive { seconds: 3. });
        }
        if clickable.clicked {
            let offset = ((mouse.position.x - 41.) / 116. + 0.5).clamp(0., 1.) * 0.9 + 0.1;
            mixer.set_master_volume(offset);
        }
        transform.translation.x = ((mixer.get_master_volume() * 1.1111 - 0.1) - 0.5) * 116.;
    }
}
