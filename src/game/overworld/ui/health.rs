use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;
use rand::{prelude::*, rngs::StdRng, SeedableRng};

const HEALTH_UI_POSITION: Vec2 = Vec2::new(-580., -300.);
const HEALTH_UI_SCALE: f32 = 0.55;

pub struct HealthUiPlugin;

impl Plugin for HealthUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<HealthUiSpawnEvent>()
            .add_system(health_ui_spawn)
            .add_system(health_ui_bottle_update);
    }
}

#[derive(Default, Clone, Copy)]
pub struct HealthUiSpawnEvent;

#[derive(Component)]
pub struct HealthBottle {
    threshold: f32,
    threshold_half: f32,
    last_health: f32,
    settings: BottleSettings,
}

fn random_bottle_settings() -> BottleSettings {
    BottleSettings {
        y: rand::random::<f32>() * 0.05 - 0.025,
        scale: Vec2::ONE * (0.95 + rand::random::<f32>() * 0.1),
        rotation: (rand::random::<f32>() * 20. - 10.).to_radians(),
    }
}

#[derive(Default, Debug, Clone, Copy)]
struct BottleSettings {
    y: f32,
    scale: Vec2,
    rotation: f32,
}

fn health_ui_spawn(
    mut ev_spawn: EventReader<HealthUiSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for _ in ev_spawn.iter() {
        commands
            .spawn_bundle(VisibilityBundle::default())
            .insert_bundle(TransformBundle::default())
            .insert(FollowCamera { offset: Vec2::ZERO })
            .insert(Transform2::new().without_pixel_perfect())
            .with_children(|parent| {
                parent
                    .spawn_bundle(Transform2Bundle {
                        transform2: Transform2::from_translation(HEALTH_UI_POSITION)
                            .with_scale(Vec2::ONE * HEALTH_UI_SCALE),
                        ..Default::default()
                    })
                    .insert_bundle(VisibilityBundle::default())
                    .with_children(|parent| {
                        let amt = 10;
                        let mut rng = StdRng::from_seed(Default::default());
                        for i in 0..amt {
                            let settings = random_bottle_settings();
                            let brightness = 0.7 + rng.gen::<f32>() * 0.3;
                            parent
                                .spawn_bundle(SpriteSheetBundle {
                                    sprite: TextureAtlasSprite {
                                        color: Color::rgb(brightness, brightness, brightness),
                                        ..Default::default()
                                    },
                                    texture_atlas: asset_library.sprite_health_bottle_atlas.clone(),
                                    ..Default::default()
                                })
                                .insert(
                                    Transform2::from_translation(Vec2::new(
                                        i as f32 * 45.,
                                        settings.y,
                                    ))
                                    .with_scale(settings.scale)
                                    .with_rotation(settings.rotation)
                                    .with_depth((
                                        DEPTH_LAYER_UI_HEALTH_BOTTLE.0,
                                        DEPTH_LAYER_UI_HEALTH_BOTTLE.1 + brightness * 0.01,
                                    )),
                                )
                                .insert(HealthBottle {
                                    threshold: if i == 0 { 0.001 } else { i as f32 / amt as f32 },
                                    threshold_half: if i == amt - 1 {
                                        1.
                                    } else {
                                        ((i as f32 / amt as f32) + ((i + 1) as f32 / amt as f32))
                                            * 0.5
                                    },
                                    last_health: 1.,
                                    settings,
                                });
                        }
                    });
            });
    }
}

fn health_ui_bottle_update(
    mut query: Query<(&mut TextureAtlasSprite, &mut Transform2, &mut HealthBottle)>,
    player_query: Query<&Health, With<Player>>,
    time: Res<Time>,
) {
    let player_health = if let Ok(health) = player_query.get_single() {
        health.value as f32 / health.max as f32
    } else {
        return;
    };
    for (mut sprite, mut transform, mut bottle) in query.iter_mut() {
        if player_health < bottle.last_health {
            bottle.settings = random_bottle_settings();
            transform.translation.y += rand::random::<f32>() * 30.;
        }
        let lerp_amt = time.delta_seconds() * 2.2_f32;
        transform.translation.y = (lerp_amt * 2.).lerp(transform.translation.y, bottle.settings.y);
        transform.scale = transform.scale.lerp(bottle.settings.scale, lerp_amt);
        transform.rotation = lerp_amt.lerp(transform.rotation, bottle.settings.rotation);
        bottle.last_health = player_health;
        if player_health < bottle.threshold {
            sprite.index = 2;
        } else if player_health < bottle.threshold_half {
            sprite.index = 1;
        } else {
            sprite.index = 0;
        }
    }
}
