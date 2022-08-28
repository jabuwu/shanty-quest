use crate::common::prelude::*;
use audio_plus::prelude::*;
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
            .add_system_set(SystemSet::on_update(AppState::IntroCutscene).with_system(skip))
            .add_system_set(SystemSet::on_update(AppState::IntroCutscene).with_system(image_move));
    }
}

#[derive(Default, Debug, Clone)]
pub struct IntroCutscene;

impl Cutscene for IntroCutscene {
    fn build(cutscene: &mut CutsceneBuilder) {
        cutscene.add_timed_step(step1_fade_in, 0.0); // 1.5s
        cutscene.add_timed_step(step1_image, 12.75);
        cutscene.add_timed_step(step1_fade_out, 2.0); // 2.5s

        cutscene.add_timed_step(step2_start_audio, 0.5); // start audio during fade out

        cutscene.add_timed_step(step2_fade_in, 0.0); // 2.5sec
        cutscene.add_timed_step(step2_image, 9.5);

        cutscene.add_timed_step(step2_fade_out, 1.0); //1s fade, Runs near "Find them they did!"

        cutscene.add_timed_step(step3_fade_in, 0.0); // 1.5sec
        cutscene.add_timed_step(step3_image, 2.0);

        cutscene.add_timed_step(step3_start_audio, 15.75);
        cutscene.add_timed_step(step3_fade_out, 1.5); //1..5s

        cutscene.add_timed_step(step4, 14.5);
        cutscene.add_timed_step(step4_fade_out, 1.5); //1..5s

        cutscene.add_timed_step(reset, 0.5);
        cutscene.add_timed_step(step5, 10.5);
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
                "".to_owned(),
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
        AudioPlusSource::new(asset_library.sound_effects.sfx_cutscene_intro_music.clone())
            .as_looping(),
    );
}

fn skip(
    mut cutscene_state: ResMut<IntroCutsceneState>,
    input: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    mut screen_fade: ResMut<ScreenFade>,
    mut ev_cutscene_skip: EventWriter<CutsceneSkipEvent<IntroCutscene>>,
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

fn reset(mut screen_fade: ResMut<ScreenFade>, state: Res<IntroCutsceneState>) {
    if !state.proceed {
        screen_fade.fade_out(0.5);
    }
}

fn end(
    mut screen_fade: ResMut<ScreenFade>,
    state: Res<IntroCutsceneState>,
    mut query: Query<&mut AudioPlusSource>,
) {
    if !state.proceed {
        screen_fade.fade_out(1.0);
    }
    for mut source in query.iter_mut() {
        source.stop();
    }
}

fn step1_fade_in(mut screen_fade: ResMut<ScreenFade>, state: Res<IntroCutsceneState>) {
    if !state.proceed {
        screen_fade.fade_in(2.5);
    }
}

fn step1_image(
    mut query: Query<&mut Text, With<CutsceneText>>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
    cutscenes: Res<Cutscenes>,
) {
    if let Ok(mut text) = query.get_single_mut() {
        text.sections[0].value = "Well, ya oiled me tongue with a jug o' rum so lemme tell ya the story of treble at sea! Eh? How the Pirate Lords became... Lords?!".to_owned();
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
                AudioPlusSource::new(asset_library.sound_effects.sfx_cutscene_intro1.clone())
                    .as_playing(),
            );
    }
}

fn step1_fade_out(mut screen_fade: ResMut<ScreenFade>, state: Res<IntroCutsceneState>) {
    if !state.proceed {
        screen_fade.fade_out(2.5);
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
            "Royal Navy beat those scurvy dogs 'gain and 'gain! Driven them to seek Rockdorado and the fabled lost weapons... Find them they did!".to_owned();
    }
    if !cutscenes.skipping() {
        commands.spawn().insert(
            AudioPlusSource::new(asset_library.sound_effects.sfx_cutscene_intro2.clone())
                .as_playing(),
        );
    }
}

fn step2_fade_in(mut screen_fade: ResMut<ScreenFade>, state: Res<IntroCutsceneState>) {
    if !state.proceed {
        screen_fade.fade_in(1.5);
    }
}

fn step2_image(
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
    cutscenes: Res<Cutscenes>,
) {
    if !cutscenes.skipping() {
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_library.cutscene_image_intro2.clone(),
                ..Default::default()
            })
            .insert(
                Transform2::from_xy(245., 145.)
                    .with_scale(Vec2::ONE * 0.44)
                    .with_depth((DepthLayer::Entity, 0.1))
                    .without_pixel_perfect(),
            )
            .insert(CutsceneImage {
                velocity: Vec2::new(-20., -6.),
            });
    }
}

fn step2_fade_out(mut screen_fade: ResMut<ScreenFade>, state: Res<IntroCutsceneState>) {
    if !state.proceed {
        screen_fade.fade_out(1.0);
    }
}

fn step3_fade_in(mut screen_fade: ResMut<ScreenFade>, state: Res<IntroCutsceneState>) {
    if !state.proceed {
        screen_fade.fade_in(1.5);
    }
}

fn step3_image(
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
    cutscenes: Res<Cutscenes>,
) {
    if !cutscenes.skipping() {
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_library.cutscene_image_intro3.clone(),
                ..Default::default()
            })
            .insert(
                Transform2::from_xy(0., -100.)
                    .with_scale(Vec2::ONE * 0.4)
                    .with_depth((DepthLayer::Entity, 0.3))
                    .without_pixel_perfect(),
            )
            .insert(CutsceneImage {
                velocity: Vec2::new(0., 4.),
            });
    }
}

fn step3_start_audio(
    mut query: Query<&mut Text, With<CutsceneText>>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
    cutscenes: Res<Cutscenes>,
) {
    if let Ok(mut text) = query.get_single_mut() {
        text.sections[0].value =
            "Each Cap'n grabbed an instrument!\nHah. Gave 'em terrible powers of horrid noise, magical projectiles, power over sea monsters! With that, they smashed the Royal Navy ta bits!".to_owned();
    }
    if !cutscenes.skipping() {
        commands.spawn().insert(
            AudioPlusSource::new(asset_library.sound_effects.sfx_cutscene_intro3.clone())
                .as_playing(),
        );
    }
}

fn step3_fade_out(mut screen_fade: ResMut<ScreenFade>, state: Res<IntroCutsceneState>) {
    if !state.proceed {
        screen_fade.fade_out(1.5);
    }
}

fn step4(
    mut query: Query<&mut Text, With<CutsceneText>>,
    mut commands: Commands,
    mut screen_fade: ResMut<ScreenFade>,
    state: Res<IntroCutsceneState>,
    asset_library: Res<AssetLibrary>,
    cutscenes: Res<Cutscenes>,
) {
    if !state.proceed {
        screen_fade.fade_in(0.5);
    }
    if let Ok(mut text) = query.get_single_mut() {
        text.sections[0].value =
            "But, right as rum, pirates be pirates. Each Lord wished to get more powa, to get all other instruments! That's how this Pirate Lords War started...".to_owned();
    }
    if !cutscenes.skipping() {
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_library.cutscene_image_intro4.clone(),
                ..Default::default()
            })
            .insert(
                Transform2::from_xy(0., 180.)
                    .with_scale(Vec2::ONE * 0.42)
                    .with_depth((DepthLayer::Entity, 0.4))
                    .without_pixel_perfect(),
            )
            .insert(CutsceneImage {
                velocity: Vec2::new(0., -15.),
            })
            .insert(
                AudioPlusSource::new(asset_library.sound_effects.sfx_cutscene_intro4.clone())
                    .as_playing(),
            );
    }
}

fn step4_fade_out(mut screen_fade: ResMut<ScreenFade>, state: Res<IntroCutsceneState>) {
    if !state.proceed {
        screen_fade.fade_out(1.5);
    }
}

fn step5(
    mut query: Query<&mut Text, With<CutsceneText>>,
    mut commands: Commands,
    mut screen_fade: ResMut<ScreenFade>,
    state: ResMut<IntroCutsceneState>,
    asset_library: Res<AssetLibrary>,
    cutscenes: Res<Cutscenes>,
) {
    if !state.proceed {
        screen_fade.fade_in(0.5);
    }
    if let Ok(mut text) = query.get_single_mut() {
        text.sections[0].value =
            "That's why the rum ships sail less and less... Now! Buy me another jug or I'll yapper no more tales.".to_owned();
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
                AudioPlusSource::new(asset_library.sound_effects.sfx_cutscene_intro5.clone())
                    .as_playing(),
            );
    }
}

fn cleanup(mut app_state: ResMut<State<AppState>>) {
    app_state.set(AppState::Overworld).unwrap();
}
