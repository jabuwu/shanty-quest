use crate::common::prelude::*;
use bevy::prelude::*;

pub struct IntroCutscenePlugin;

impl Plugin for IntroCutscenePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::IntroCutscene).with_system(intro_cutscene_init),
        )
        .add_system_set(
            SystemSet::on_update(AppState::IntroCutscene).with_system(intro_cutscene_update),
        );
    }
}

fn intro_cutscene_init(
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
    mut screen_fade: ResMut<ScreenFade>,
) {
    screen_fade.fade_in(1.);

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
        .insert(Transform2::from_xy(0., 0.).with_depth((DepthLayer::Front, 0.)));
}

fn intro_cutscene_update(
    mut keys: ResMut<Input<KeyCode>>,
    mut app_state: ResMut<State<AppState>>,
    mut screen_fade: ResMut<ScreenFade>,
) {
    if keys.just_pressed(KeyCode::Space) {
        app_state.set(AppState::Overworld).unwrap();
        keys.reset(KeyCode::Space);
        screen_fade.set(1.);
    }
}
