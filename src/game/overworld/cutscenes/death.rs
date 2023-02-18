use crate::common::prelude::*;
use audio_plus::prelude::*;
use bevy::prelude::*;

const DEATH_SECONDS: f32 = 1.;

pub struct DeathCutscenePlugin;

impl Plugin for DeathCutscenePlugin {
    fn build(&self, app: &mut App) {
        app.add_cutscene::<DeathCutscene>();
    }
}

#[derive(Default, Debug, Clone, Resource)]
pub struct DeathCutscene;

impl Cutscene for DeathCutscene {
    fn build(cutscene: &mut CutsceneBuilder) {
        cutscene.add_timed_step(init1, DEATH_SECONDS);
        cutscene.add_timed_step(cleanup, DEATH_SECONDS);
    }
}

fn init1(
    mut screen_fade: ResMut<ScreenFade>,
    asset_library: Res<AssetLibrary>,
    mut commands: Commands,
) {
    screen_fade.fade_out(0.3);
    commands
        .spawn(Transform2Bundle::default())
        .insert(
            AudioPlusSource::new(
                asset_library
                    .sound_effects
                    .sfx_overworld_player_died
                    .clone(),
            )
            .as_playing(),
        )
        .insert(TimeToLive { seconds: 8. })
        .insert(Persistent);
}

fn cleanup(mut app_state: ResMut<State<AppState>>) {
    app_state.set(AppState::Dead).unwrap();
}
