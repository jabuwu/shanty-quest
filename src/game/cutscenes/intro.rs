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
            .add_system_set(
                SystemSet::on_enter(AppState::IntroCutscene).with_system(intro_cutscene_init),
            )
            .add_system_set(
                SystemSet::on_update(AppState::IntroCutscene).with_system(intro_cutscene_update),
            );
    }
}

fn intro_cutscene_init(
    mut cutscene_state: ResMut<IntroCutsceneState>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
    mut screen_fade: ResMut<ScreenFade>,
) {
    *cutscene_state = IntroCutsceneState::default();
    screen_fade.fade_in(1.);

    commands.spawn_bundle(Camera2dBundle::default());
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section(
                "Intro cutscene!\n\nLeft click to skip".to_owned(),
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
        .insert(Transform2::from_xy(0., 0.).with_depth((DepthLayer::Front, 0.)));
}

fn intro_cutscene_update(
    mut cutscene_state: ResMut<IntroCutsceneState>,
    input: Res<Input<MouseButton>>,
    mut app_state: ResMut<State<AppState>>,
    mut screen_fade: ResMut<ScreenFade>,
) {
    if input.just_pressed(MouseButton::Left) {
        cutscene_state.proceed = true;
        screen_fade.fade_out(1.);
    }
    if screen_fade.faded_out() && cutscene_state.proceed {
        app_state.set(AppState::Overworld).unwrap();
    }
}
