use crate::common::prelude::*;
use asset_struct::AssetStruct;
use bevy::prelude::*;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Loading).with_system(loading_init))
            .add_system_set(SystemSet::on_update(AppState::Loading).with_system(loading_update));
    }
}

fn loading_init(
    mut commands: Commands,
    mut asset_library: ResMut<AssetLibrary>,
    mut texture_atlas_assets: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    asset_library.load_assets(&asset_server);
    asset_library.create_texture_atlases(texture_atlas_assets.as_mut());
    asset_library.create_sound_effects();
    commands.spawn_bundle(Camera2dBundle::default());
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section(
                "Loading".to_owned(),
                TextStyle {
                    font: asset_library.font_default.clone(),
                    font_size: 68.0,
                    color: Color::WHITE,
                },
            )
            .with_alignment(TextAlignment {
                horizontal: HorizontalAlign::Center,
                vertical: VerticalAlign::Center,
            }),
            ..Default::default()
        })
        .insert(Transform2::new().with_depth((DepthLayer::Front, 0.)));
}

pub fn loading_update(
    mut app_state: ResMut<State<AppState>>,
    asset_library: Res<AssetLibrary>,
    asset_server: Res<AssetServer>,
    mut screen_fade: ResMut<ScreenFade>,
) {
    use bevy::asset::LoadState;
    match asset_library.load_state(&asset_server) {
        LoadState::Failed => {
            panic!("Failed to load assets.");
        }
        LoadState::Loaded => {
            if !screen_fade.fading() {
                screen_fade.enable();
                screen_fade.set(0.);
                screen_fade.fade_out(0.1);
            }
            if screen_fade.faded_out() {
                app_state.set(AppState::MainMenu).unwrap();
            }
        }
        _ => {}
    }
}
