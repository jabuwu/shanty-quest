use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;

pub struct OctopusPlugin;

impl Plugin for OctopusPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OctopusSpawnEvent>()
            .add_system(octopus_spawn)
            .add_system(octopus_move)
            .add_system(octopus_animate)
            .add_system(octopus_invincibility);
    }
}

#[derive(Default, Clone, Copy)]
pub struct OctopusSpawnEvent {
    pub position: Vec2,
}

#[derive(Component)]
pub struct Octopus;

fn octopus_spawn(
    mut ev_spawn: EventReader<OctopusSpawnEvent>,
    mut commands: Commands,
    mut ev_healthbar_spawn: EventWriter<HealthbarSpawnEvent>,
    asset_library: Res<AssetLibrary>,
) {
    for event in ev_spawn.iter() {
        let entity = commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: asset_library.sprite_octopus_atlas.clone(),
                ..Default::default()
            })
            .insert(
                Transform2::from_translation(event.position).with_depth((DepthLayer::Entity, 0.)),
            )
            .insert(Octopus)
            .insert(Label("Octopus".to_owned()))
            .insert(YDepth::default())
            .insert(Health::new(10.))
            .insert(Hitbox {
                shape: CollisionShape::Rect {
                    size: Vec2::new(60., 60.),
                },
                for_entity: None,
            })
            .insert(Hurtbox {
                shape: CollisionShape::Rect {
                    size: Vec2::new(80., 80.),
                },
                for_entity: None,
                auto_despawn: false,
            })
            .insert(Collision {
                shape: CollisionShape::Rect {
                    size: Vec2::new(60., 60.),
                },
                flags: COLLISION_FLAG,
            })
            .insert(CharacterController {
                movement: Vec2::ZERO,
                speed: 100.,
            })
            .insert(AutoDamage {
                despawn: true,
                ..Default::default()
            })
            .id();
        ev_healthbar_spawn.send(HealthbarSpawnEvent {
            entity: Some(entity),
            offset: Vec2::new(0., 75.),
            size: Vec2::new(80., 6.),
        });
    }
}

fn octopus_move(mut query: Query<&mut CharacterController, With<Octopus>>) {
    for mut character_controller in query.iter_mut() {
        let mut angle = Vec2::X.angle_between(character_controller.movement);
        if angle.is_nan() {
            angle = 0.;
        }
        angle += rand::random::<f32>() * 0.2;
        angle -= rand::random::<f32>() * 0.2;
        character_controller.movement = Vec2::from_angle(angle);
    }
}

fn octopus_animate(mut query: Query<&mut TextureAtlasSprite, With<Octopus>>, time: Res<Time>) {
    for mut sprite in query.iter_mut() {
        let time = time.time_since_startup().as_secs_f32() % 1.;
        if time > 0.5 {
            sprite.index = 1;
        } else {
            sprite.index = 0;
        }
    }
}

fn octopus_invincibility(mut query: Query<(&mut TextureAtlasSprite, &AutoDamage)>) {
    for (mut sprite, auto_damage) in query.iter_mut() {
        if auto_damage.invincibility > 0. {
            sprite.color.set_a(0.5);
        } else {
            sprite.color.set_a(1.);
        };
    }
}
