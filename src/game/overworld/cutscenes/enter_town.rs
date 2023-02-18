use crate::common::prelude::*;
use crate::game::prelude::*;
use audio_plus::prelude::*;
use bevy::prelude::*;

const ENTER_TOWN_TIME_SECONDS: f32 = 1.4;

#[derive(Default, Resource)]
pub struct EnterTownCutsceneState {
    time: f32,
}

pub struct EnterTownCutscenePlugin;

impl Plugin for EnterTownCutscenePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnterTownCutsceneState>()
            .add_cutscene::<EnterTownCutscene>();
    }
}

#[derive(Default, Debug, Clone, Resource)]
pub struct EnterTownCutscene {
    pub boat: Option<Entity>,
    pub from: Vec2,
    pub to: Vec2,
}

impl Cutscene for EnterTownCutscene {
    fn build(cutscene: &mut CutsceneBuilder) {
        cutscene.add_step(init1, update1.before(OverworldCameraSystems::Update));
        cutscene.add_quick_step(cleanup);
    }
}

fn init1(
    mut state: ResMut<EnterTownCutsceneState>,
    mut screen_fade: ResMut<ScreenFade>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
    mut ev_sound_stop: EventWriter<WorldAmbienceSoundStopEvent>,
) {
    *state = EnterTownCutsceneState::default();
    screen_fade.fade_out(ENTER_TOWN_TIME_SECONDS);
    commands.spawn_empty().insert(
        AudioPlusSource::new(asset_library.sound_effects.sfx_overworld_town_enter.clone())
            .as_playing(),
    );
    ev_sound_stop.send_default();
}

fn update1(
    cutscene: Res<EnterTownCutscene>,
    mut query: Query<&mut Transform2>,
    mut state: ResMut<EnterTownCutsceneState>,
    time: Res<Time>,
    mut ev_continue: EventWriter<CutsceneContinueEvent<EnterTownCutscene>>,
) {
    state.time += time.delta_seconds() / ENTER_TOWN_TIME_SECONDS;
    state.time = state.time.clamp(0.03, 1.);
    if let Some(entity) = cutscene.boat {
        if let Ok(mut transform) = query.get_mut(entity) {
            let easing = ease(Easing::CubicOut, state.time);
            transform.scale = Vec2::ONE * easing.lerp(1., 0.1);
            let to = cutscene.from.lerp(cutscene.to, easing);
            transform.translation = transform
                .translation
                .lerp(to, (state.time * 2.).clamp(0., 1.));
        }
    }
    if state.time == 1. {
        ev_continue.send_default();
    }
}

fn cleanup(mut app_state: ResMut<State<AppState>>) {
    app_state.set(AppState::TownOutside).unwrap();
}
