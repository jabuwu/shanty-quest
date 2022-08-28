use crate::common::prelude::*;
use bevy::prelude::*;

pub struct DamageRumPlugin;

impl Plugin for DamageRumPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DamageRumSpawnEvent>()
            .add_system(damage_rum_spawn)
            .add_system(damage_rum_update);
    }
}

#[derive(Default, Clone, Copy)]
pub struct DamageRumSpawnEvent {
    pub position: Vec2,
}

#[derive(Component)]
struct DamageRum {
    velocity: Vec2,
    angular: f32,
    scale: f32,
    opacity: f32,
}

impl DamageRum {
    fn new() -> Self {
        Self {
            velocity: Vec2::from_angle(rand::random::<f32>() * std::f32::consts::TAU) * 550.
                + rand::random::<f32>() * 250.,
            angular: (rand::random::<f32>() * 2. - 1.)
                * (10. + (rand::random::<f32>().powf(2.)) * 40.),
            scale: 0.7 + rand::random::<f32>() * 0.3,
            opacity: 0.7 + rand::random::<f32>() * 0.3,
        }
    }
}

fn damage_rum_spawn(
    mut ev_spawn: EventReader<DamageRumSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for event in ev_spawn.iter() {
        let bottles = 2 + rand::random::<u32>() % 5;
        for _ in 0..bottles {
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(1., 1., 1., 0.),
                        ..Default::default()
                    },
                    texture: asset_library.sprite_rum_bottle.clone(),
                    ..Default::default()
                })
                .insert(
                    Transform2::from_translation(event.position)
                        .with_depth((DepthLayer::Entity, 1.))
                        .with_scale(Vec2::ONE * 0.25),
                )
                .insert(DamageRum::new());
        }
    }
}

fn damage_rum_update(
    mut query: Query<(Entity, &mut Transform2, &mut Sprite, &mut DamageRum)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, mut transform, mut sprite, mut rum) in query.iter_mut() {
        transform.translation += rum.velocity * time.delta_seconds();
        transform.rotation += rum.angular * time.delta_seconds();
        transform.scale = Vec2::ONE * (1. - ease(Easing::BackIn, rum.opacity)) * 0.3 * rum.scale;
        sprite.color.set_a(ease(Easing::BackOut, rum.opacity));
        rum.velocity *= 0.003_f32.powf(time.delta_seconds());
        rum.angular *= 0.3_f32.powf(time.delta_seconds());
        rum.opacity -= time.delta_seconds();
        if rum.opacity < 0. {
            commands.entity(entity).despawn();
        }
    }
}
