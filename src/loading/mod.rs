use crate::common::prelude::*;
use asset_struct::AssetStruct;
use bevy::{prelude::*, sprite::Anchor};

#[derive(Default, Resource)]
struct LoadingState {
    fading: bool,
}

#[derive(Component)]
struct LoadingText;

#[derive(Component)]
struct LoadingProgress;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LoadingState>()
            .add_system(loading_init.in_schedule(OnEnter(AppState::Loading)))
            .add_system(loading_update.in_set(OnUpdate(AppState::Loading)));
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
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(Text2dBundle {
            text: Text::from_section(
                "Loading".to_owned(),
                TextStyle {
                    font: asset_library.font_bold.clone(),
                    font_size: 68.0,
                    color: Color::WHITE,
                },
            )
            .with_alignment(TextAlignment::Center),
            text_anchor: Anchor::Center,
            ..Default::default()
        })
        .insert(Transform2::new().with_depth((DepthLayer::Front, 0.)))
        .insert(LoadingText);
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Vec2::new(160., 12.).into(),
                color: Color::BLACK,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Transform2::from_xy(0., -70.).with_depth((DepthLayer::Front, 0.)));
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Vec2::new(160., 12.).into(),
                color: Color::WHITE,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(
            Transform2::from_xy(0., -70.)
                .with_scale(Vec2::new(0., 1.))
                .with_depth((DepthLayer::Front, 0.1)),
        )
        .insert(LoadingProgress);
}

fn loading_update(
    mut app_state: ResMut<NextState<AppState>>,
    asset_library: Res<AssetLibrary>,
    asset_server: Res<AssetServer>,
    mut screen_fade: ResMut<ScreenFade>,
    mut ev_dialogue_init: EventWriter<DialogueInitEvent>,
    mut state: ResMut<LoadingState>,
    mut text_query: Query<&mut Text, With<LoadingText>>,
    mut progress_query: Query<(&mut Transform2, &mut Sprite), With<LoadingProgress>>,
) {
    use bevy::asset::LoadState;
    let failed = match asset_library.load_state(&asset_server) {
        LoadState::Failed => {
            for mut text in text_query.iter_mut() {
                text.sections[0].value = "Failed to load assets.".to_owned();
                text.sections[0].style.color = Color::rgb(1., 0.3, 0.3);
            }
            for (mut progress_transform, mut sprite) in progress_query.iter_mut() {
                progress_transform.scale.x = 1.;
                progress_transform.translation.x = 0.;
                sprite.color = Color::rgb(1., 0.3, 0.3);
            }
            true
        }
        LoadState::Loaded => {
            if state.fading && screen_fade.faded_out() {
                app_state.set(AppState::MainMenu);
                ev_dialogue_init.send_default();
            }
            if !state.fading {
                screen_fade.enable();
                screen_fade.set(0.);
                screen_fade.fade_out(0.1);
                state.fading = true;
            }
            false
        }
        _ => false,
    };
    if !failed {
        let progress = asset_library.load_progress(&asset_server);
        for (mut progress_transform, _) in progress_query.iter_mut() {
            progress_transform.scale.x = progress;
            progress_transform.translation.x = -80. * (1. - progress);
        }
    }
}
