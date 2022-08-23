use crate::common::prelude::*;
use bevy::prelude::*;

pub struct ScreenFadePlugin;

impl Plugin for ScreenFadePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ScreenFade {
            enabled: false,
            opacity: 1.,
            speed: 0.,
            state: ScreenFadeState::Fading,
        })
        .add_startup_system(screen_fade_init)
        .add_system(screen_fade_update);
    }
}

pub struct ScreenFade {
    enabled: bool,
    opacity: f32,
    speed: f32,
    state: ScreenFadeState,
}

enum ScreenFadeState {
    Fading,
    FadedIn,
    FadedOut,
}

impl ScreenFade {
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn fade_in(&mut self, seconds: f32) {
        self.speed = -1. / seconds;
    }

    pub fn fade_out(&mut self, seconds: f32) {
        self.speed = 1. / seconds;
    }

    pub fn faded_in(&self) -> bool {
        matches!(self.state, ScreenFadeState::FadedIn)
    }

    pub fn faded_out(&self) -> bool {
        matches!(self.state, ScreenFadeState::FadedOut)
    }

    pub fn set(&mut self, opacity: f32) {
        self.opacity = opacity;
    }
}

#[derive(Component)]
struct ScreenFadeComp;

fn screen_fade_init(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Vec2::new(50000., 50000.).into(),
                color: Color::BLACK,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Transform2::new().with_depth((DepthLayer::Front, 1.)))
        .insert(ScreenFadeComp)
        .insert(Persistent);
}

fn screen_fade_update(
    mut query: Query<&mut Sprite, With<ScreenFadeComp>>,
    mut screen_fade: ResMut<ScreenFade>,
    time: Res<Time>,
) {
    for mut sprite in query.iter_mut() {
        if !screen_fade.enabled {
            sprite.color.set_a(0.);
        } else {
            screen_fade.opacity += time.delta_seconds() * screen_fade.speed;
            screen_fade.opacity = screen_fade.opacity.clamp(0., 1.);
            sprite
                .color
                .set_a(ease(Easing::CubicInOut, screen_fade.opacity));
            if screen_fade.opacity == 1. {
                screen_fade.state = ScreenFadeState::FadedOut;
            } else if screen_fade.opacity == 0. {
                screen_fade.state = ScreenFadeState::FadedIn;
            } else {
                screen_fade.state = ScreenFadeState::Fading;
            }
        }
    }
}
