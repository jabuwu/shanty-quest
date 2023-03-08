use crate::common::prelude::*;
use crate::game::prelude::*;
use audio_plus::prelude::*;
use bevy::prelude::*;

const BACK_FADE: f32 = 0.36;

#[derive(Default, Resource)]
struct RumRefillState {
    opacity: f32,
    time: f32,
    index: u32,
}

pub struct RumRefillPlugin;

impl Plugin for RumRefillPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RumRefillState>()
            .add_cutscene::<RumRefillCutscene>();
    }
}

#[derive(Component)]
struct RumRefillBg;

#[derive(Component)]
struct RumRefillParent;

#[derive(Component)]
struct RumRefillBottle {
    index: u32,
    to: Vec2,
    from: Vec2,
}

#[derive(Default, Debug, Clone, Resource)]
pub struct RumRefillCutscene;

impl Cutscene for RumRefillCutscene {
    fn build(cutscene: &mut CutsceneBuilder) {
        cutscene.add_dialogue_step(|| {});
        cutscene.add_step(init1, update1);
        cutscene.add_step(|| {}, update2);
        cutscene.add_step(|| {}, update3);
        cutscene.add_quick_step(cleanup);
    }
}

fn init1(
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
    mut state: ResMut<RumRefillState>,
    game_state: Res<GameState>,
) {
    *state = RumRefillState::default();
    let health = game_state.health / game_state.health_max;
    state.index = 1 + (health * 8.).round() as u32;
    commands.spawn((
        Transform2Bundle::default(),
        AudioPlusSource::new(
            asset_library
                .sound_effects
                .sfx_town_rum_refill_jingle
                .clone(),
        )
        .as_playing(),
        TimeToLive { seconds: 3. },
    ));
    commands
        .spawn((
            TransformBundle::default(),
            VisibilityBundle::default(),
            Transform2::new().with_scale(Vec2::ONE * 1.5),
            RumRefillParent,
        ))
        .with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Vec2::new(9999., 9999.).into(),
                        color: Color::rgba(1., 1., 1., 0.),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Transform2::new().with_depth(DEPTH_LAYER_TOWN_OUTSIDE_RUM_REFILL_BG),
                RumRefillBg,
            ));
            for i in 0..10 {
                let x = (i as f32 - 4.5) * 40.;
                let brightness = 0.6 + rand::random::<f32>() * 0.4;
                let to = Vec2::new(x, 0.);
                let from = Vec2::new(x + 100., 0.);
                parent.spawn((
                    SpriteSheetBundle {
                        sprite: TextureAtlasSprite {
                            color: Color::rgba(brightness, brightness, brightness, 0.),
                            ..Default::default()
                        },
                        texture_atlas: asset_library.sprite_health_bottle_atlas.clone(),
                        ..Default::default()
                    },
                    Transform2::from_translation(to)
                        .with_rotation(rand::random::<f32>() * 0.2 - 0.1)
                        .with_scale(Vec2::ONE * (1. + rand::random::<f32>() * 0.2 - 0.1))
                        .with_depth((
                            DEPTH_LAYER_TOWN_OUTSIDE_RUM_REFILL_BOTTLE.0,
                            DEPTH_LAYER_TOWN_OUTSIDE_RUM_REFILL_BOTTLE.1 + brightness * 0.001,
                        )),
                    RumRefillBottle { index: i, to, from },
                ));
            }
        });
}

fn update1(
    mut bg_query: Query<&mut Sprite, With<RumRefillBg>>,
    mut bottle_query: Query<(&mut TextureAtlasSprite, &RumRefillBottle)>,
    time: Res<Time>,
    mut ev_continue: EventWriter<CutsceneContinueEvent<RumRefillCutscene>>,
    mut state: ResMut<RumRefillState>,
) {
    state.opacity += time.delta_seconds() * 3.;
    state.opacity = state.opacity.clamp(0., 1.);
    for mut sprite in bg_query.iter_mut() {
        sprite.color.set_a(state.opacity * BACK_FADE);
    }
    for (mut sprite, bottle) in bottle_query.iter_mut() {
        if state.index > bottle.index {
            sprite.color.set_a(state.opacity);
        }
    }
    if state.opacity == 1. {
        ev_continue.send_default();
    }
}

fn update2(
    mut ev_continue: EventWriter<CutsceneContinueEvent<RumRefillCutscene>>,
    mut state: ResMut<RumRefillState>,
    mut bottle_query: Query<(&mut TextureAtlasSprite, &mut Transform2, &RumRefillBottle)>,
    time: Res<Time>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    state.time += time.delta_seconds() * 5.5;
    state.time = state.time.clamp(0., 1.);
    for (mut sprite, mut transform, bottle) in bottle_query.iter_mut() {
        if bottle.index == state.index {
            sprite.color.set_a(state.time);
            transform.translation = bottle
                .from
                .lerp(bottle.to, ease(Easing::BounceOut, state.time));
        }
    }
    if state.time == 1. {
        state.index += 1;
        if state.index == 10 {
            ev_continue.send_default();
        } else {
            state.time = 0.
        }
        commands.spawn((
            Transform2Bundle::default(),
            AudioPlusSource::new(
                asset_library
                    .sound_effects
                    .sfx_town_rum_refill_clank
                    .clone(),
            )
            .as_playing(),
            TimeToLive { seconds: 3. },
        ));
    }
}

fn update3(
    mut bg_query: Query<&mut Sprite, With<RumRefillBg>>,
    mut bottle_query: Query<&mut TextureAtlasSprite, With<RumRefillBottle>>,
    time: Res<Time>,
    mut ev_continue: EventWriter<CutsceneContinueEvent<RumRefillCutscene>>,
    mut state: ResMut<RumRefillState>,
) {
    state.opacity -= time.delta_seconds() * 3.;
    state.opacity = state.opacity.clamp(0., 1.);
    for mut sprite in bg_query.iter_mut() {
        sprite.color.set_a(state.opacity * BACK_FADE);
    }
    for mut sprite in bottle_query.iter_mut() {
        sprite.color.set_a(state.opacity);
    }
    if state.opacity == 0. {
        ev_continue.send_default();
    }
}

fn cleanup(
    mut commands: Commands,
    query: Query<Entity, With<RumRefillParent>>,
    mut game_state: ResMut<GameState>,
) {
    game_state.health = game_state.health_max;
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
