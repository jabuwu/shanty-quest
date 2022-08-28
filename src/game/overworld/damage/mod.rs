use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;

pub const DAMAGE_FLAG_PLAYER: u32 = 1;
pub const DAMAGE_FLAG_ENEMY: u32 = 2;
pub const DAMAGE_FLAG_ENVIRONMENT: u32 = 4;

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DamageEvent>()
            .add_system(damage_check)
            .add_system(damage_auto_die);
    }
}

#[derive(Clone, Copy)]
pub struct DamageEvent {
    pub hit: Entity,
    pub hurt: Entity,
    pub damage: f32,
}

#[derive(Component)]
pub struct Hitbox {
    pub shape: CollisionShape,
    pub for_entity: Option<Entity>,
    pub flags: u32,
}

#[derive(Component)]
pub struct Hurtbox {
    pub shape: CollisionShape,
    pub for_entity: Option<Entity>,
    pub auto_despawn: bool,
    pub flags: u32,
    pub knockback_type: HurtboxKnockbackType,
    pub damage: f32,
}

#[derive(Debug, Clone, Copy)]
pub enum HurtboxKnockbackType {
    None,
    Velocity(Vec2),
    Difference(f32),
}

#[derive(Component, Default)]
pub struct AutoDamage {
    pub despawn: bool,
    pub invincibility: f32,
    pub invincibility_amount: f32,
    pub already_despawned: bool,
    pub experience: f32,
    pub experience_count: u32,
    pub experience_infinite_distance: bool,
}

fn damage_check(
    hitbox_query: Query<(Entity, &Hitbox)>,
    hurtbox_query: Query<(Entity, &Hurtbox)>,
    transform_query: Query<&GlobalTransform>,
    mut ev_damage: EventWriter<DamageEvent>,
    mut commands: Commands,
    cutscenes: Res<Cutscenes>,
    mut ev_knockback: EventWriter<KnockbackEvent>,
) {
    if cutscenes.running() {
        return;
    }
    for (hurtbox_entity, hurtbox) in hurtbox_query.iter() {
        let hurtbox_translation = if let Ok(transform) = transform_query.get(hurtbox_entity) {
            transform.translation().truncate()
        } else {
            continue;
        };
        let hurt = if let Some(proxy_entity) = hurtbox.for_entity {
            proxy_entity
        } else {
            hurtbox_entity
        };
        let mut despawn = false;
        for (hitbox_entity, hitbox) in hitbox_query.iter() {
            let hit = if let Some(proxy_entity) = hitbox.for_entity {
                proxy_entity
            } else {
                hitbox_entity
            };
            if hit == hurt {
                continue;
            }
            if hurtbox.flags & hitbox.flags == 0 {
                continue;
            }
            let hitbox_translation = if let Ok(transform) = transform_query.get(hitbox_entity) {
                transform.translation().truncate()
            } else {
                continue;
            };
            if hitbox
                .shape
                .overlaps(hitbox_translation, hurtbox.shape, hurtbox_translation)
            {
                match hurtbox.knockback_type {
                    HurtboxKnockbackType::Velocity(force) => {
                        ev_knockback.send(KnockbackEvent {
                            entity: hitbox_entity,
                            force,
                        });
                    }
                    HurtboxKnockbackType::Difference(mult_force) => {
                        let difference = hitbox_translation - hurtbox_translation;
                        let force = (1.0 - (difference.length() / 500.).clamp(0., 1.)) * mult_force;
                        ev_knockback.send(KnockbackEvent {
                            entity: hitbox_entity,
                            force: difference.normalize() * force,
                        });
                    }
                    _ => {}
                }
                ev_damage.send(DamageEvent {
                    hit,
                    hurt,
                    damage: hurtbox.damage,
                });
                if hurtbox.auto_despawn {
                    despawn = true;
                    break;
                }
            }
        }
        if despawn {
            commands.entity(hurtbox_entity).despawn_recursive();
        }
    }
}

fn damage_auto_die(
    mut ev_damage: EventReader<DamageEvent>,
    mut crate_query: Query<(Entity, &mut Health, &mut AutoDamage, &GlobalTransform)>,
    mut commands: Commands,
    time: Res<Time>,
    cutscenes: Res<Cutscenes>,
    mut ev_experience_spawn: EventWriter<ExperienceSpawnEvent>,
) {
    for (_, _, mut auto_damage, _) in crate_query.iter_mut() {
        auto_damage.invincibility -= time.delta_seconds();
        auto_damage.invincibility = auto_damage.invincibility.max(0.);
    }
    for event in ev_damage.iter() {
        if let Ok((entity, mut health, mut auto_damage, transform)) = crate_query.get_mut(event.hit)
        {
            if auto_damage.invincibility == 0. {
                auto_damage.invincibility_amount = 0.
            }
            if event.damage > auto_damage.invincibility_amount {
                if !cutscenes.running() {
                    health.damage(event.damage - auto_damage.invincibility_amount);
                }
                auto_damage.invincibility = 0.1;
                auto_damage.invincibility_amount = event.damage;
            }
            if health.dead() && !auto_damage.already_despawned {
                commands.entity(entity).despawn_recursive();
                if auto_damage.experience > 0. {
                    ev_experience_spawn.send(ExperienceSpawnEvent {
                        amount: auto_damage.experience,
                        position: transform.translation().truncate(),
                        count: auto_damage.experience_count,
                        infinite_distance: auto_damage.experience_infinite_distance,
                    });
                }
                auto_damage.already_despawned = true;
            }
        }
    }
}
