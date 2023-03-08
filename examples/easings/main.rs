use asset_struct::prelude::*;
use bevy::{prelude::*, sprite::Anchor, window::WindowResolution};
use jam::common::prelude::*;

const EASING_COUNT: usize = 30;
const EASINGS: [Easing; EASING_COUNT] = [
    Easing::SineIn,
    Easing::SineOut,
    Easing::SineInOut,
    Easing::QuadIn,
    Easing::QuadOut,
    Easing::QuadInOut,
    Easing::CubicIn,
    Easing::CubicOut,
    Easing::CubicInOut,
    Easing::QuartIn,
    Easing::QuartOut,
    Easing::QuartInOut,
    Easing::QuintIn,
    Easing::QuintOut,
    Easing::QuintInOut,
    Easing::ExpoIn,
    Easing::ExpoOut,
    Easing::ExpoInOut,
    Easing::CircIn,
    Easing::CircOut,
    Easing::CircInOut,
    Easing::BackIn,
    Easing::BackOut,
    Easing::BackInOut,
    Easing::ElasticIn,
    Easing::ElasticOut,
    Easing::ElasticInOut,
    Easing::BounceIn,
    Easing::BounceOut,
    Easing::BounceInOut,
];

#[derive(Component)]
pub struct Editable;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Easings".to_string(),
                resolution: WindowResolution::new(1280., 720.),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(CommonPlugin)
        .add_startup_system(init)
        .add_system(update)
        .run();
}

#[derive(Component)]
struct EasingComp {
    function: Easing,
    time: f32,
    cooldown: f32,
}

pub fn init(
    mut commands: Commands,
    mut asset_library: ResMut<AssetLibrary>,
    asset_server: Res<AssetServer>,
) {
    asset_library.load_assets(&asset_server);
    commands.spawn(Camera2dBundle::default());
    for (i, easing) in EASINGS.iter().enumerate() {
        let x = (i % 10) as f32 * 120. - 550.;
        let y = (i / 10) as f32 * -200. + 170.;
        commands
            .spawn((
                VisibilityBundle::default(),
                TransformBundle::default(),
                Transform2::from_xy(x, y),
            ))
            .with_children(|parent| {
                parent.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Vec2::new(16., 16.).into(),
                            color: Color::RED,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    EasingComp {
                        function: *easing,
                        time: 0.,
                        cooldown: 0.,
                    },
                    Transform2::new(),
                ));
                parent.spawn((
                    Text2dBundle {
                        text: Text::from_section(
                            format!("{:?}", *easing),
                            TextStyle {
                                font: asset_library.font_default.clone(),
                                font_size: 24.0,
                                color: Color::WHITE,
                            },
                        )
                        .with_alignment(TextAlignment::Center),
                        text_anchor: Anchor::Center,
                        ..Default::default()
                    },
                    Transform2::from_xy(0., 100.),
                ));
            });
    }
}

fn update(mut query: Query<(&mut Transform2, &mut EasingComp)>, time: Res<Time>) {
    for (mut transform, mut easing) in query.iter_mut() {
        easing.time = (easing.time + time.delta_seconds()).clamp(0., 1.);
        transform.translation.y = ease(easing.function, easing.time) * 100. - 50.;
        if easing.time == 1. {
            easing.cooldown += time.delta_seconds();
            if easing.cooldown > 0.5 {
                easing.time = 0.;
                easing.cooldown = 0.;
            }
        }
    }
}
