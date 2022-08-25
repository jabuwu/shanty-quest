use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;

const OCTOPUS_COLLISION_SIZE: Vec2 = Vec2::new(60., 60.);
const OCTOPUS_HURTBOX_SIZE: Vec2 = Vec2::new(80., 80.);

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
    pub entity: Option<Entity>,
    pub position: Vec2,
}

#[derive(Component)]
pub struct Octopus;

fn octopus_spawn(
    mut ev_spawn: EventReader<OctopusSpawnEvent>,
    mut commands: Commands,
    mut ev_healthbar_spawn: EventWriter<HealthbarSpawnEvent>,
    asset_library: Res<AssetLibrary>,
    collision_query: Res<CollisionQuery>,
) {
    for event in ev_spawn.iter() {
        if collision_query
            .check(
                event.position,
                CollisionShape::Rect {
                    size: OCTOPUS_COLLISION_SIZE * 1.5,
                },
                None,
            )
            .is_some()
        {
            continue;
        }
        let mut entity = if let Some(entity) = event.entity {
            commands.entity(entity)
        } else {
            commands.spawn()
        };
        entity
            .insert_bundle(SpriteSheetBundle {
                texture_atlas: asset_library.sprite_octopus_atlas.clone(),
                ..Default::default()
            })
            .insert(
                Transform2::from_translation(event.position).with_depth((DepthLayer::Entity, 0.)),
            )
            .insert(Octopus)
            .insert(Label("Octopus".to_owned()))
            .insert(YDepth::default())
            .insert(Health::new(3.))
            .insert(Hitbox {
                shape: CollisionShape::Rect {
                    size: Vec2::new(60., 60.),
                },
                for_entity: None,
                flags: DAMAGE_FLAG_ENEMY,
            })
            .insert(Hurtbox {
                shape: CollisionShape::Rect {
                    size: OCTOPUS_HURTBOX_SIZE,
                },
                for_entity: None,
                auto_despawn: false,
                flags: DAMAGE_FLAG_PLAYER,
                knockback_type: HurtboxKnockbackType::None,
            })
            .insert(Collision {
                shape: CollisionShape::Rect {
                    size: OCTOPUS_COLLISION_SIZE,
                },
                flags: COLLISION_FLAG,
            })
            .insert(CharacterController {
                movement: Vec2::ZERO,
                speed: 150.,
                ..Default::default()
            })
            .insert(AutoDamage {
                despawn: true,
                ..Default::default()
            });
        ev_healthbar_spawn.send(HealthbarSpawnEvent {
            entity: Some(entity.id()),
            offset: Vec2::new(0., 75.),
            size: Vec2::new(80., 6.),
        });
    }
}

fn octopus_move(
    mut queries: ParamSet<(
        Query<(&mut CharacterController, &GlobalTransform), With<Octopus>>,
        Query<&GlobalTransform, With<Player>>,
    )>,
    cutscenes: Res<Cutscenes>,
) {
    let player_position = if let Ok(player_transform) = queries.p1().get_single() {
        player_transform.translation().truncate()
    } else {
        Vec2::ZERO
    };
    for (mut character_controller, octopus_transform) in queries.p0().iter_mut() {
        if cutscenes.running() {
            character_controller.movement = Vec2::ZERO;
        } else {
            let direction = player_position - octopus_transform.translation().truncate();
            character_controller.movement = direction.normalize();
        }
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
