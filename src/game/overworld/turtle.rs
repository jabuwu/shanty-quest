use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;

const TURTLE_COLLISION_SIZE: Vec2 = Vec2::new(60., 60.);
const TURTLE_HURTBOX_SIZE: Vec2 = Vec2::new(80., 80.);

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum TurtleSystem {
    Spawn,
}

pub struct TurtlePlugin;

impl Plugin for TurtlePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TurtleSpawnEvent>()
            .add_system(
                turtle_spawn
                    .in_set(TurtleSystem::Spawn)
                    .before(HealthbarSystem::Spawn),
            )
            .add_system(turtle_move)
            .add_system(turtle_animate);
    }
}

#[derive(Default, Clone, Copy)]
pub struct TurtleSpawnEvent {
    pub entity: Option<Entity>,
    pub position: Vec2,
    pub level: TurtleLevel,
}

#[derive(Default, Clone, Copy)]
pub enum TurtleLevel {
    #[default]
    Easy,
    Medium,
    Hard,
}

impl TurtleLevel {
    fn info(&self, asset_library: &AssetLibrary) -> TurtleInfo {
        match *self {
            Self::Easy => TurtleInfo {
                atlas: asset_library.sprite_turtle_easy_atlas.clone(),
                scale: 1.0,
                health: 15.,
                speed: 150.,
                knockback_resistence: 0.8,
                experience: 1.,
                experience_count: 4,
            },
            Self::Medium => TurtleInfo {
                atlas: asset_library.sprite_turtle_medium_atlas.clone(),
                scale: 0.7,
                health: 20.,
                speed: 450.,
                knockback_resistence: 0.0,
                experience: 1.,
                experience_count: 8,
            },
            Self::Hard => TurtleInfo {
                atlas: asset_library.sprite_turtle_hard_atlas.clone(),
                scale: 1.5,
                health: 50.,
                speed: 100.,
                knockback_resistence: 1.0,
                experience: 4.,
                experience_count: 6,
            },
        }
    }
}

struct TurtleInfo {
    atlas: Handle<TextureAtlas>,
    scale: f32,
    health: f32,
    speed: f32,
    knockback_resistence: f32,
    experience: f32,
    experience_count: u32,
}

#[derive(Component)]
pub struct Turtle {
    relative_angle: f32,
    sprite_angle: f32,
}

#[derive(Component)]
pub struct TurtleSprite;

fn turtle_spawn(
    mut ev_spawn: EventReader<TurtleSpawnEvent>,
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
                    size: TURTLE_COLLISION_SIZE * 1.5,
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
            commands.spawn_empty()
        };
        let info = event.level.info(asset_library.as_ref());
        entity
            .insert((
                TransformBundle::default(),
                VisibilityBundle::default(),
                Transform2::from_translation(event.position),
                Turtle {
                    relative_angle: rand::random::<f32>() * std::f32::consts::TAU,
                    sprite_angle: 0.,
                },
                YDepth::default(),
                Health::new(info.health),
                Hitbox {
                    shape: CollisionShape::Rect {
                        size: Vec2::new(80., 80.) * info.scale,
                    },
                    for_entity: None,
                    flags: DAMAGE_FLAG_ENEMY,
                },
                Hurtbox {
                    shape: CollisionShape::Rect {
                        size: TURTLE_HURTBOX_SIZE,
                    },
                    for_entity: None,
                    auto_despawn: false,
                    flags: DAMAGE_FLAG_PLAYER,
                    knockback_type: HurtboxKnockbackType::None,
                    damage: 1.,
                },
                Collision {
                    shape: CollisionShape::Rect {
                        size: TURTLE_COLLISION_SIZE * info.scale,
                    },
                    flags: COLLISION_FLAG,
                },
                CharacterController {
                    movement: Vec2::ZERO,
                    speed: info.speed,
                    knockback_resistance: info.knockback_resistence,
                    ..Default::default()
                },
                AutoDamage {
                    despawn: true,
                    experience: info.experience,
                    experience_count: info.experience_count,
                    ..Default::default()
                },
            ))
            .with_children(|parent| {
                parent.spawn((
                    SpriteSheetBundle {
                        texture_atlas: info.atlas,
                        ..Default::default()
                    },
                    Transform2::new()
                        .with_depth((DepthLayer::Entity, 0.))
                        .with_scale(Vec2::ONE * info.scale),
                    TurtleSprite,
                ));
            });
        ev_healthbar_spawn.send(HealthbarSpawnEvent {
            entity: Some(entity.id()),
            offset: Vec2::new(0., 75.),
            size: Vec2::new(80., 6.),
        });
    }
}

fn turtle_move(
    mut queries: ParamSet<(
        Query<(&mut CharacterController, &GlobalTransform, &mut Turtle)>,
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
    for (mut character_controller, turtle_transform, mut turtle) in queries.p0().iter_mut() {
        if cutscenes.running() {
            character_controller.movement = Vec2::ZERO;
        } else {
            let distance = turtle_transform
                .translation()
                .truncate()
                .distance(player_position);
            //turtle.relative_angle += rand::random::<f32>() * std::f32::consts::PI * 0.01;
            let mut direction = (player_position
                + Vec2::from_angle(turtle.relative_angle) * (40. + distance * 0.7))
                - turtle_transform.translation().truncate();
            if direction.length() == 0. {
                direction = Vec2::ONE;
            }
            turtle.sprite_angle = (time.delta_seconds() * 2.).lerp(
                turtle.sprite_angle,
                turtle.sprite_angle
                    + Vec2::from_angle(turtle.sprite_angle).angle_between(direction.normalize()),
            );
            character_controller.movement = Vec2::from_angle(turtle.sprite_angle);
        }
    }
}

fn turtle_animate(
    mut query: Query<(&Turtle, &Children, &AutoDamage)>,
    mut child_query: Query<(&mut TextureAtlasSprite, &mut Transform2), With<TurtleSprite>>,
    time: Res<Time>,
) {
    for (turtle, children, auto_damage) in query.iter_mut() {
        for child in children.iter() {
            if let Ok((mut sprite, mut transform)) = child_query.get_mut(*child) {
                transform.rotation = turtle.sprite_angle + std::f32::consts::PI * 1.3;
                let time = (time.elapsed_seconds() * 2.) % 1.;
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
