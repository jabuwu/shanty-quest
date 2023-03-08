use crate::common::prelude::*;
use crate::game::prelude::*;
use audio_plus::prelude::*;
use bevy::prelude::*;

const ENTER_TOWN_TIME_SECONDS: f32 = 0.6;

#[derive(Default, Resource)]
pub struct ExitTownCutsceneState {
    time: f32,
}

pub struct ExitTownCutscenePlugin;

impl Plugin for ExitTownCutscenePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ExitTownCutsceneState>()
            .add_cutscene::<ExitTownCutscene>();
    }
}

#[derive(Default, Debug, Clone, Resource)]
pub struct ExitTownCutscene {
    pub boat: Option<Entity>,
    pub from: Vec2,
    pub to: Vec2,
}

impl Cutscene for ExitTownCutscene {
    fn build(cutscene: &mut CutsceneBuilder) {
        cutscene.add_step(init1, update1.before(OverworldCameraSet::Update));
    }
}

fn init1(
    mut state: ResMut<ExitTownCutsceneState>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    *state = ExitTownCutsceneState::default();
    commands.spawn(
        AudioPlusSource::new(asset_library.sound_effects.sfx_overworld_town_exit.clone())
            .as_playing(),
    );
}

fn update1(
    cutscene: Res<ExitTownCutscene>,
    mut query: Query<&mut Transform2>,
    mut state: ResMut<ExitTownCutsceneState>,
    time: Res<Time>,
    mut ev_continue: EventWriter<CutsceneContinueEvent<ExitTownCutscene>>,
) {
    state.time += time.delta_seconds() / ENTER_TOWN_TIME_SECONDS;
    state.time = state.time.clamp(0.03, 1.);
    if let Some(entity) = cutscene.boat {
        if let Ok(mut transform) = query.get_mut(entity) {
            let time = 1. - state.time;
            let easing = ease(Easing::CubicOut, time);
            transform.scale = Vec2::ONE * easing.lerp(1., 0.1);
            let to = cutscene.to.lerp(cutscene.from, easing);
            transform.translation = transform.translation.lerp(to, (time * 2.).clamp(0., 1.));
        }
    }
    if state.time == 1. {
        ev_continue.send_default();
    }
}
