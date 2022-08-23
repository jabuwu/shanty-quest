use crate::common::prelude::*;
use bevy::prelude::*;

#[derive(Default)]
struct IntroCutsceneState {
    proceed: bool,
}

pub struct IntroCutscenePlugin;

impl Plugin for IntroCutscenePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<IntroCutsceneState>()
            .add_cutscene::<IntroCutscene>()
            .add_system_set(SystemSet::on_enter(AppState::IntroCutscene).with_system(init))
            .add_system_set(SystemSet::on_update(AppState::IntroCutscene).with_system(skip));
    }
}

#[derive(Default, Debug)]
pub struct IntroCutscene;

impl Cutscene for IntroCutscene {
    fn build(cutscene: &mut CutsceneBuilder) {
        cutscene.add_timed_step(step1, 2.5);
        cutscene.add_timed_step(step2, 2.5);
        cutscene.add_timed_step(step3, 2.5);
        cutscene.add_quick_step(cleanup);
    }
}

#[derive(Component)]
struct CutsceneText;

fn init(
    mut cutscene_state: ResMut<IntroCutsceneState>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
    mut screen_fade: ResMut<ScreenFade>,
    mut ev_cutscene_start: EventWriter<CutsceneStartEvent<IntroCutscene>>,
) {
    *cutscene_state = IntroCutsceneState::default();
    screen_fade.fade_in(1.);

    ev_cutscene_start.send_default();

    commands.spawn_bundle(Camera2dBundle::default());
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section(
                "Intro cutscene!\n\nPress space to skip".to_owned(),
                TextStyle {
                    font: asset_library.font_default.clone(),
                    font_size: 32.0,
                    color: Color::WHITE,
                },
            )
            .with_alignment(TextAlignment {
                horizontal: HorizontalAlign::Center,
                vertical: VerticalAlign::Center,
            }),
            ..Default::default()
        })
        .insert(Transform2::from_xy(0., 0.).with_depth((DepthLayer::Front, 0.)))
        .insert(CutsceneText);
}

fn skip(
    mut cutscene_state: ResMut<IntroCutsceneState>,
    input: Res<Input<KeyCode>>,
    mut screen_fade: ResMut<ScreenFade>,
    mut ev_cutscene_skip: EventWriter<CutsceneSkipEvent<IntroCutscene>>,
) {
    if input.just_pressed(KeyCode::Space) {
        cutscene_state.proceed = true;
        screen_fade.fade_out(1.);
    }
    if screen_fade.faded_out() && cutscene_state.proceed {
        ev_cutscene_skip.send_default();
    }
}

fn step1(mut query: Query<&mut Text, With<CutsceneText>>) {
    if let Ok(mut text) = query.get_single_mut() {
        text.sections[0].value = "This is a cutscene!".to_owned();
    }
}

fn step2(mut query: Query<&mut Text, With<CutsceneText>>) {
    if let Ok(mut text) = query.get_single_mut() {
        text.sections[0].value =
            "You can wait for it to finish.\nOr press space to skip.".to_owned();
    }
}

fn step3(
    mut query: Query<&mut Text, With<CutsceneText>>,
    mut cutscene_state: ResMut<IntroCutsceneState>,
    mut screen_fade: ResMut<ScreenFade>,
) {
    if let Ok(mut text) = query.get_single_mut() {
        text.sections[0].value = "Enjoy the game".to_owned();
    }
    cutscene_state.proceed = true;
    screen_fade.fade_out(2.5);
}

fn cleanup(mut app_state: ResMut<State<AppState>>) {
    app_state.set(AppState::Overworld).unwrap();
}
