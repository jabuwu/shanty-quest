use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;

const OCTOPUS_COLLISION_SIZE: Vec2 = Vec2::new(60., 60.);
const OCTOPUS_HURTBOX_SIZE: Vec2 = Vec2::new(80., 80.);

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum OctopusSystems {
    Spawn,
}

pub struct OctopusPlugin;

impl Plugin for OctopusPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OctopusSpawnEvent>()
            .add_system(
                octopus_spawn
                    .label(OctopusSystems::Spawn)
                    .before(HealthbarSystems::Spawn),
            )
            .add_system(octopus_move)
            .add_system(octopus_animate);
    }
}

#[derive(Default, Clone, Copy)]
pub struct OctopusSpawnEvent {
    pub entity: Option<Entity>,
    pub position: Vec2,
    pub level: OctopusLevel,
}

#[derive(Default, Clone, Copy)]
pub enum OctopusLevel {
    #[default]
    Easy,
    Medium,
    Hard,
}

impl OctopusLevel {
    fn info(&self, asset_library: &AssetLibrary) -> OctopusInfo {
        match *self {
            Self::Easy => OctopusInfo {
                atlas: asset_library.sprite_octopus_easy_atlas.clone(),
                scale: 0.75,
                health: 2.5,
                speed: 150.,
                knockback_resistence: 0.,
                experience: 1.,
                experience_count: 2,
            },
            Self::Medium => OctopusInfo {
                atlas: asset_library.sprite_octopus_medium_atlas.clone(),
                scale: 1.0,
                health: 3.5,
                speed: 300.,
                knockback_resistence: 0.4,
                experience: 1.,
                experience_count: 5,
            },
            Self::Hard => OctopusInfo {
                atlas: asset_library.sprite_octopus_hard_atlas.clone(),
                scale: 1.2,
                health: 5.,
                speed: 150.,
                knockback_resistence: 0.7,
                experience: 3.,
                experience_count: 3,
            },
        }
    }
}

struct OctopusInfo {
    atlas: Handle<TextureAtlas>,
    scale: f32,
    health: f32,
    speed: f32,
    knockback_resistence: f32,
    experience: f32,
    experience_count: u32,
}

#[derive(Component)]
pub struct Octopus {
    stop_chance: TimedChance,
    stop_time: f32,
}

#[derive(Component)]
pub struct OctopusSprite;

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
        let OctopusInfo {
            atlas,
            scale,
            health,
            speed,
            knockback_resistence,
            experience,
            experience_count,
        } = event.level.info(asset_library.as_ref());
        entity
            .insert_bundle(TransformBundle::default())
            .insert_bundle(VisibilityBundle::default())
            .insert(Transform2::from_translation(event.position))
            .insert(Octopus {
                stop_chance: TimedChance::new(),
                stop_time: 0.,
            })
            .insert(YDepth::default())
            .insert(Health::new(health))
            .insert(Hitbox {
                shape: CollisionShape::Rect {
                    size: Vec2::new(60., 60.) * scale,
                },
                for_entity: None,
                flags: DAMAGE_FLAG_ENEMY,
            })
            .insert(Hurtbox {
                shape: CollisionShape::Rect {
                    size: OCTOPUS_HURTBOX_SIZE * scale,
                },
                for_entity: None,
                auto_despawn: false,
                flags: DAMAGE_FLAG_PLAYER,
                knockback_type: HurtboxKnockbackType::None,
                damage: 1.,
            })
            .insert(Collision {
                shape: CollisionShape::Rect {
                    size: OCTOPUS_COLLISION_SIZE,
                },
                flags: COLLISION_FLAG,
            })
            .insert(CharacterController {
                movement: Vec2::ZERO,
                speed: speed,
                knockback_resistance: knockback_resistence,
                ..Default::default()
            })
            .insert(AutoDamage {
                despawn: true,
                experience,
                experience_count,
                ..Default::default()
            })
            .with_children(|parent| {
                parent
                    .spawn_bundle(SpriteSheetBundle {
                        texture_atlas: atlas,
                        ..Default::default()
                    })
                    .insert(
                        Transform2::new()
                            .with_depth((DepthLayer::Entity, 0.))
                            .with_scale(Vec2::ONE * scale),
                    )
                    .insert(OctopusSprite);
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
        Query<(&mut CharacterController, &GlobalTransform, &mut Octopus)>,
        Query<&GlobalTransform, With<Player>>,
    )>,
    cutscenes: Res<Cutscenes>,
    time: Res<Time>,
) {
    let player_position = if let Ok(player_transform) = queries.p1().get_single() {
        player_transform.translation().truncate()
    } else {
        Vec2::ZERO
    };
    for (mut character_controller, octopus_transform, mut octopus) in queries.p0().iter_mut() {
        if octopus.stop_time < 0. && octopus.stop_chance.check(6., 3., time.delta_seconds()) {
            octopus.stop_time = 0.5;
        }
        octopus.stop_time -= time.delta_seconds();
        if cutscenes.running() || octopus.stop_time > 0. {
            character_controller.movement = Vec2::ZERO;
        } else {
            let direction = player_position - octopus_transform.translation().truncate();
            character_controller.movement = direction.normalize();
        }
    }
}

fn octopus_animate(
    query: Query<(&Children, &AutoDamage), With<Octopus>>,
    mut child_query: Query<&mut TextureAtlasSprite, With<OctopusSprite>>,
    time: Res<Time>,
) {
    for (children, auto_damage) in query.iter() {
        for child in children.iter() {
            if let Ok(mut sprite) = child_query.get_mut(*child) {
                let time = time.time_since_startup().as_secs_f32() % 1.;
                if time > 0.5 {
                    sprite.index = 1;
                } else {
                    sprite.index = 0;
                }
                if auto_damage.invincibility > 0. {
                    sprite.color.set_a(0.5);
                } else {
                    sprite.color.set_a(1.);
                };
            }
        }
    }
}
