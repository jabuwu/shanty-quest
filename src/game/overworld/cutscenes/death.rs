use crate::common::prelude::*;
use bevy::prelude::*;

const DEATH_SECONDS: f32 = 0.3;

pub struct DeathCutscenePlugin;

impl Plugin for DeathCutscenePlugin {
    fn build(&self, app: &mut App) {
        app.add_cutscene::<DeathCutscene>();
    }
}

#[derive(Default, Debug, Clone)]
pub struct DeathCutscene;

impl Cutscene for DeathCutscene {
    fn build(cutscene: &mut CutsceneBuilder) {
        cutscene.add_timed_step(init1, DEATH_SECONDS);
        cutscene.add_timed_step(cleanup, DEATH_SECONDS);
    }
}

fn init1(mut screen_fade: ResMut<ScreenFade>) {
    screen_fade.fade_out(DEATH_SECONDS);
}

fn cleanup(mut app_state: ResMut<State<AppState>>) {
    app_state.set(AppState::Dead).unwrap();
}
