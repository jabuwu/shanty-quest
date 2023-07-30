use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;

pub struct HealthAuraPlugin;

impl Plugin for HealthAuraPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<HealthAuraSpawnEvent>()
            .add_systems(Update, (health_aura_spawn, health_aura_update));
    }
}

#[derive(Event, Default, Clone, Copy)]
pub struct HealthAuraSpawnEvent;

#[derive(Component)]
pub struct HealthAura {
    sides: bool,
}

fn health_aura_spawn(
    mut ev_spawn: EventReader<HealthAuraSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for _ in ev_spawn.iter() {
        commands
            .spawn((
                VisibilityBundle::default(),
                TransformBundle::default(),
                FollowCamera { offset: Vec2::ZERO },
                Transform2::new().without_pixel_perfect(),
            ))
            .with_children(|parent| {
                parent.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Vec2::new(1280., 768.).into(),
                            color: Color::RED,
                            ..Default::default()
                        },
                        texture: asset_library.sprite_screen_edges.clone(),
                        ..Default::default()
                    },
                    Transform2::from_xy(0., 0.)
                        .with_depth(DEPTH_LAYER_HEALTH_AURA_1)
                        .without_pixel_perfect(),
                    HealthAura { sides: true },
                ));
                parent.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Vec2::new(1280., 768.).into(),
                            color: Color::rgb(0.1, 0., 0.),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Transform2::from_xy(0., 0.)
                        .with_depth(DEPTH_LAYER_HEALTH_AURA_2)
                        .without_pixel_perfect(),
                    HealthAura { sides: false },
                ));
            });
    }
}

fn health_aura_update(
    mut query: Query<(&mut Sprite, &HealthAura)>,
    player_query: Query<&Health, With<Player>>,
    time: Res<Time>,
) {
    let health = if let Ok(health) = player_query.get_single() {
        health.value / health.max
    } else {
        1.
    };
    let pulse = 1. + (time.elapsed_seconds() * 3.).sin() * 0.3;
    for (mut sprite, aura) in query.iter_mut() {
        if aura.sides {
            let intensity = 1.0 - (health * 2.5).clamp(0., 1.);
            sprite.color.set_a(intensity * pulse);
        } else {
            let intensity = (1.0 - (health * 15.).clamp(0., 1.)).clamp(0., 0.3);
            sprite.color.set_a(intensity * pulse);
        }
    }
}
