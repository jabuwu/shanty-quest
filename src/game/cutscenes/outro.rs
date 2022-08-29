use crate::common::prelude::*;
use crate::game::prelude::*;
use audio_plus::prelude::*;
use bevy::prelude::*;

#[derive(Default)]
struct OutroCutsceneState {
    proceed: bool,
}

pub struct OutroCutscenePlugin;

impl Plugin for OutroCutscenePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<OutroCutsceneState>()
            .add_cutscene::<OutroCutscene>()
            .add_system_set(SystemSet::on_enter(AppState::OutroCutscene).with_system(init))
            .add_system_set(SystemSet::on_update(AppState::OutroCutscene).with_system(skip))
            .add_system_set(SystemSet::on_update(AppState::OutroCutscene).with_system(image_move));
    }
}

#[derive(Default, Debug, Clone)]
pub struct OutroCutscene;

impl Cutscene for OutroCutscene {
    fn build(cutscene: &mut CutsceneBuilder) {
        cutscene.add_timed_step(step1, 12.5); // 13s
        cutscene.add_timed_step(step1_fade_out, 0.5); // 1.5s fade
        cutscene.add_timed_step(step2_start_audio, 1.0); // start audio during fade out

        cutscene.add_timed_step(step2, 8.5); // 9s
        cutscene.add_timed_step(step2_fade_out, 1.0);
        cutscene.add_timed_step(step3, 9.); // 9
        cutscene.add_timed_step(end, 1.0);
        cutscene.add_quick_step(cleanup);
    }
}

#[derive(Component)]
struct CutsceneText;

#[derive(Component)]
struct CutsceneImage {
    velocity: Vec2,
}

fn init(
    mut cutscene_state: ResMut<OutroCutsceneState>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
    mut screen_fade: ResMut<ScreenFade>,
    mut ev_cutscene_start: EventWriter<CutsceneStartEvent<OutroCutscene>>,
) {
    *cutscene_state = OutroCutsceneState::default();
    screen_fade.fade_in(1.);

    ev_cutscene_start.send_default();

    commands.spawn_bundle(Camera2dBundle::default());
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section(
                "Outro cutscene!\n\nPress space to skip".to_owned(),
                TextStyle {
                    font: asset_library.font_default.clone(),
                    font_size: 24.0,
                    color: Color::WHITE,
                },
            )
            .with_alignment(TextAlignment {
                horizontal: HorizontalAlign::Center,
                vertical: VerticalAlign::Center,
            }),
            ..Default::default()
        })
        .insert(Transform2::from_xy(0., -300.).with_depth((DepthLayer::Front, 1.)))
        .insert(CutsceneText);
    commands.spawn().insert(
        AudioPlusSource::new(asset_library.sound_effects.sfx_cutscene_outro_music.clone())
            .as_looping(),
    );
}

fn skip(
    mut cutscene_state: ResMut<OutroCutsceneState>,
    input: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    mut screen_fade: ResMut<ScreenFade>,
    mut ev_cutscene_skip: EventWriter<CutsceneSkipEvent<OutroCutscene>>,
    mut query: Query<&mut AudioPlusSource>,
) {
    if input.just_pressed(KeyCode::Space) || mouse.just_pressed(MouseButton::Left) {
        if !cutscene_state.proceed {
            cutscene_state.proceed = true;
            screen_fade.fade_out(1.);
            for mut source in query.iter_mut() {
                source.stop();
            }
        }
    }
    if screen_fade.faded_out() && cutscene_state.proceed {
        ev_cutscene_skip.send_default();
    }
}

fn image_move(mut query: Query<(&mut Transform2, &CutsceneImage)>, time: Res<Time>) {
    for (mut transform, image) in query.iter_mut() {
        transform.translation += image.velocity * time.delta_seconds();
    }
}

fn end(
    mut screen_fade: ResMut<ScreenFade>,
    state: Res<OutroCutsceneState>,
    mut query: Query<&mut AudioPlusSource>,
) {
    if !state.proceed {
        screen_fade.fade_out(1.0);
    }
    for mut source in query.iter_mut() {
        source.stop();
    }
}

fn step1(
    mut query: Query<&mut Text, With<CutsceneText>>,
    mut commands: Commands,
    mut screen_fade: ResMut<ScreenFade>,
    state: Res<OutroCutsceneState>,
    asset_library: Res<AssetLibrary>,
    cutscenes: Res<Cutscenes>,
) {
    if !state.proceed {
        screen_fade.fade_in(0.5);
    }
    if let Ok(mut text) = query.get_single_mut() {
        text.sections[0].value = "And there ya go, laddie! That is when the sea trebled! When a Pirate Lord combined all the magical instruments and became a Pirate King!".to_owned();
    }
    if !cutscenes.skipping() {
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_library.cutscene_image_intro1.clone(),
                ..Default::default()
            })
            .insert(
                Transform2::from_xy(200., -50.)
                    .with_scale(Vec2::ONE * 0.65)
                    .with_depth((DepthLayer::Entity, 0.0))
                    .without_pixel_perfect(),
            )
            .insert(CutsceneImage {
                velocity: Vec2::new(-6., -6.),
            })
            .insert(
                AudioPlusSource::new(asset_library.sound_effects.sfx_cutscene_outro1.clone())
                    .as_playing(),
            );
    }
}
fn step1_fade_out(mut screen_fade: ResMut<ScreenFade>, state: Res<OutroCutsceneState>) {
    if !state.proceed {
        screen_fade.fade_out(1.5);
    }
}

fn step2_start_audio(
    mut query: Query<&mut Text, With<CutsceneText>>,
    cutscenes: Res<Cutscenes>,
    asset_library: Res<AssetLibrary>,
    mut commands: Commands,
) {
    if let Ok(mut text) = query.get_single_mut() {
        text.sections[0].value =
            "Now he be raiding the coast with the most horrible noise known to mankind..."
                .to_owned();
    }
    if !cutscenes.skipping() {
        commands.spawn().insert(
            AudioPlusSource::new(asset_library.sound_effects.sfx_cutscene_outro2.clone())
                .as_playing(),
        );
    }
}

fn step2(
    mut commands: Commands,
    mut screen_fade: ResMut<ScreenFade>,
    state: Res<OutroCutsceneState>,
    asset_library: Res<AssetLibrary>,
    cutscenes: Res<Cutscenes>,
) {
    if !state.proceed {
        screen_fade.fade_in(1.5);
    }
    if !cutscenes.skipping() {
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_library.cutscene_image_outro2.clone(),
                ..Default::default()
            })
            .insert(
                Transform2::from_xy(0., -50.)
                    .with_scale(Vec2::ONE * 0.42)
                    .with_depth((DepthLayer::Entity, 0.4))
                    .without_pixel_perfect(),
            )
            .insert(CutsceneImage {
                velocity: Vec2::new(0., -15.),
            });
    }
}

fn step2_fade_out(mut screen_fade: ResMut<ScreenFade>, state: Res<OutroCutsceneState>) {
    if !state.proceed {
        screen_fade.fade_out(1.0);
    }
}

fn step3(
    mut query: Query<&mut Text, With<CutsceneText>>,
    mut commands: Commands,
    mut screen_fade: ResMut<ScreenFade>,
    state: Res<OutroCutsceneState>,
    asset_library: Res<AssetLibrary>,
    cutscenes: Res<Cutscenes>,
) {
    if !state.proceed {
        screen_fade.fade_in(1.0);
    }
    if let Ok(mut text) = query.get_single_mut() {
        text.sections[0].value =
            "Buy Ol' Nipper here another jug o' rum and I'll yapper until the sunrise! Har-har!"
                .to_owned();
    }
    if !cutscenes.skipping() {
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_library.cutscene_image_intro1.clone(),
                ..Default::default()
            })
            .insert(
                Transform2::from_xy(-150., -100.)
                    .with_scale(Vec2::ONE * 0.65)
                    .with_depth((DepthLayer::Entity, 0.5))
                    .without_pixel_perfect(),
            )
            .insert(CutsceneImage {
                velocity: Vec2::new(8., 6.),
            })
            .insert(
                AudioPlusSource::new(asset_library.sound_effects.sfx_cutscene_outro3.clone())
                    .as_playing(),
            );
    }
}

fn cleanup(
    mut app_state: ResMut<State<AppState>>,
    mut game_state: ResMut<GameState>,
    world_locations: Res<WorldLocations>,
) {
    game_state.town = TownData::build("Republic of Roll", world_locations.as_ref());
    app_state.set(AppState::TownOutside).unwrap();
}
