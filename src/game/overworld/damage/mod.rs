use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;

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
}

#[derive(Component)]
pub struct Hitbox {
    pub shape: CollisionShape,
    pub for_entity: Option<Entity>,
}

#[derive(Component)]
pub struct Hurtbox {
    pub shape: CollisionShape,
    pub for_entity: Option<Entity>,
    pub auto_despawn: bool,
}

#[derive(Component, Default)]
pub struct AutoDamage {
    pub despawn: bool,
    pub invincibility: f32,
}

fn damage_check(
    hitbox_query: Query<(Entity, &Hitbox)>,
    hurtbox_query: Query<(Entity, &Hurtbox)>,
    transform_query: Query<&GlobalTransform>,
    mut ev_damage: EventWriter<DamageEvent>,
    mut commands: Commands,
) {
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
            let hitbox_translation = if let Ok(transform) = transform_query.get(hitbox_entity) {
                transform.translation().truncate()
            } else {
                continue;
            };
            if hitbox
                .shape
                .overlaps(hitbox_translation, hurtbox.shape, hurtbox_translation)
            {
                ev_damage.send(DamageEvent { hit, hurt });
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
    mut crate_query: Query<(Entity, &mut Health, &mut AutoDamage)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (_, _, mut auto_damage) in crate_query.iter_mut() {
        auto_damage.invincibility -= time.delta_seconds();
        auto_damage.invincibility = auto_damage.invincibility.max(0.);
    }
    for event in ev_damage.iter() {
        if let Ok((entity, mut health, mut auto_damage)) = crate_query.get_mut(event.hit) {
            if auto_damage.invincibility == 0. {
                health.damage(1.);
                auto_damage.invincibility = 0.1;
            }
            if health.dead() {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
