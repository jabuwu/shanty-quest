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
        .add_systems(Startup, screen_fade_init)
        .add_systems(Update, screen_fade_update);
    }
}

#[derive(Resource)]
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
        !self.enabled || (matches!(self.state, ScreenFadeState::FadedIn) && !self.fading())
    }

    pub fn faded_out(&self) -> bool {
        matches!(self.state, ScreenFadeState::FadedOut) && !self.fading()
    }

    pub fn fading(&self) -> bool {
        self.speed != 0.
    }

    pub fn set(&mut self, opacity: f32) {
        self.opacity = opacity;
    }
}

#[derive(Component)]
struct ScreenFadeComp;

fn screen_fade_init(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Vec2::new(50000., 50000.).into(),
                color: Color::BLACK,
                ..Default::default()
            },
            ..Default::default()
        },
        Transform2::new().with_depth(DEPTH_LAYER_SCREEN_FADE),
        ScreenFadeComp,
        Persistent,
    ));
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
            let dt = time.delta_seconds().min(1. / 30.);
            screen_fade.opacity += dt * screen_fade.speed;
            screen_fade.opacity = screen_fade.opacity.clamp(0., 1.);
            sprite.color.set_a(ease(
                Easing::CubicInOut,
                (screen_fade.opacity * 1.1).clamp(0., 1.),
            ));
            if screen_fade.opacity == 1. {
                screen_fade.state = ScreenFadeState::FadedOut;
                screen_fade.speed = 0.;
            } else if screen_fade.opacity == 0. {
                screen_fade.state = ScreenFadeState::FadedIn;
                screen_fade.speed = 0.;
            } else {
                screen_fade.state = ScreenFadeState::Fading;
            }
        }
    }
}
