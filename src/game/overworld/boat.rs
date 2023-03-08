use crate::common::{label::Label, prelude::*};
use crate::game::prelude::*;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

const RING_SPAWN_INTEVAL: f32 = 0.15;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum BoatSet {
    Spawn,
    Update,
}

pub struct BoatPlugin;

impl Plugin for BoatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BoatSpawnEvent>()
            .add_system(boat_spawn.before(HealthbarSet::Spawn))
            .add_system(
                boat_update
                    .in_set(BoatSet::Update)
                    .in_set(CharacterControllerSet::Update),
            )
            .add_system(boat_attack)
            .add_system(boat_debug);
    }
}

#[derive(Clone)]
pub struct BoatSpawnEvent {
    pub entity: Option<Entity>,
    pub position: Vec2,
    pub attack: Attacks,
    pub attack_cooldown: f32,
    pub healthbar: bool,
    pub player: bool,
    pub health: f32,
    pub health_max: f32,
    pub speed: f32,
    pub knockback_resistance: f32,
    pub texture_atlas: Handle<TextureAtlas>,
}

#[derive(Component)]
pub struct Boat {
    pub movement: Vec2,
    pub direction: f32,
    pub speed: f32,
    pub facing: Facing,
    pub ring_timer: f32,
    pub attacks: Attacks,
    pub shoot: bool,
    pub shoot_cooldown: f32,
    pub shoot_cooldown_threshold: f32,
    pub dash: bool,
    pub dash_cooldown: f32,
    pub opacity: f32,
}

#[derive(Component)]
pub struct BoatSprite;

fn boat_spawn(
    mut ev_spawn: EventReader<BoatSpawnEvent>,
    mut commands: Commands,
    mut ev_healthbar_spawn: EventWriter<HealthbarSpawnEvent>,
) {
    for event in ev_spawn.iter() {
        let sprite_entity = commands
            .spawn(SpriteSheetBundle {
                texture_atlas: event.texture_atlas.clone(),
                ..Default::default()
            })
            .insert(
                Transform2::new()
                    .with_depth((DepthLayer::Entity, 0.))
                    .with_scale(Vec2::new(0.6, 0.6)),
            )
            .insert(BoatSprite)
            .insert(YDepth::default())
            .id();
        let mut boat_entity = if let Some(entity) = event.entity {
            commands.entity(entity)
        } else {
            commands.spawn_empty()
        };
        let hurt_flags = if event.player {
            DAMAGE_FLAG_ENEMY | DAMAGE_FLAG_ENVIRONMENT
        } else {
            DAMAGE_FLAG_PLAYER
        };
        boat_entity
            .insert(TransformBundle::default())
            .insert(VisibilityBundle::default())
            .insert(Transform2::from_translation(event.position))
            .insert(Boat {
                movement: Vec2::ZERO,
                direction: std::f32::consts::PI * -0.5,
                speed: event.speed,
                facing: Facing::South,
                ring_timer: RING_SPAWN_INTEVAL,
                attacks: event.attack,
                shoot_cooldown: 0.,
                shoot_cooldown_threshold: event.attack_cooldown,
                shoot: false,
                dash_cooldown: 0.,
                dash: false,
                opacity: 1.,
            })
            .insert(Collision {
                shape: CollisionShape::Rect {
                    size: Vec2::new(100., 100.),
                },
                flags: COLLISION_FLAG,
            })
            .insert(CharacterController {
                movement: Vec2::ZERO,
                speed: event.speed,
                knockback_resistance: event.knockback_resistance,
                arena_adjustment: true,
                ..Default::default()
            })
            .insert(ForwardCannons {
                shoot: false,
                hurt_flags,
                level: ForwardCannonsLevel(event.attack.forward_cannons),
            })
            .insert(ShotgunCannons {
                shoot: false,
                hurt_flags,
                level: ShotgunCannonsLevel(event.attack.shotgun_cannons),
            })
            .insert(Shockwave {
                shoot: false,
                hurt_flags,
                level: ShockwaveLevel(event.attack.shockwave),
            })
            .insert(Bombs {
                shoot: false,
                hurt_flags,
                level: BombsLevel(event.attack.bombs),
            })
            .insert(Kraken {
                shoot: false,
                hurt_flags,
                level: KrakenLevel(event.attack.kraken),
            })
            .insert(DashAttack {
                shoot: false,
                hurt_flags,
            })
            .insert(Health::new_with_max(event.health, event.health_max))
            .insert(Hitbox {
                shape: CollisionShape::Rect {
                    size: Vec2::new(120., 120.),
                },
                for_entity: None,
                flags: if event.player {
                    DAMAGE_FLAG_PLAYER
                } else {
                    DAMAGE_FLAG_ENEMY
                },
            })
            .add_child(sprite_entity);
        if event.healthbar {
            ev_healthbar_spawn.send(HealthbarSpawnEvent {
                entity: Some(boat_entity.id()),
                offset: Vec2::new(0., 125.),
                size: Vec2::new(120., 8.),
            });
        }
    }
}

fn boat_update(
    mut query: Query<(
        &Transform2,
        &mut CharacterController,
        &GlobalTransform,
        &mut Boat,
        &Children,
    )>,
    mut children_query: Query<&mut TextureAtlasSprite, With<BoatSprite>>,
    time: Res<Time>,
    mut ev_water_ring_spawn: EventWriter<WaterRingSpawnEvent>,
) {
    for (transform, mut character_controller, global_transform, mut boat, children) in
        query.iter_mut()
    {
        boat.movement = boat.movement.clamp(Vec2::NEG_ONE, Vec2::ONE);
        character_controller.movement = boat.movement;
        character_controller.speed = boat.speed;
        if let Some(facing) = Facing::from_vec(Vec2::from_angle(boat.direction)) {
            boat.facing = facing;
        }
        if let Some(facing) = character_controller.force_facing {
            boat.facing = facing;
        }
        for child in children.iter() {
            if let Ok(mut atlas) = children_query.get_mut(*child) {
                atlas.color.set_a(boat.opacity);
                match boat.facing {
                    Facing::North => {
                        atlas.index = 3;
                        atlas.flip_x = false;
                    }
                    Facing::NorthEast => {
                        atlas.index = 1;
                        atlas.flip_x = true;
                    }
                    Facing::East => {
                        atlas.index = 2;
                        atlas.flip_x = true;
                    }
                    Facing::SouthEast => {
                        atlas.index = 0;
                        atlas.flip_x = true;
                    }
                    Facing::South => {
                        atlas.index = 4;
                        atlas.flip_x = false;
                    }
                    Facing::SouthWest => {
                        atlas.index = 0;
                        atlas.flip_x = false;
                    }
                    Facing::West => {
                        atlas.index = 2;
                        atlas.flip_x = false;
                    }
                    Facing::NorthWest => {
                        atlas.index = 1;
                        atlas.flip_x = false;
                    }
                }
            }
        }

        if boat.movement.length() > 0.5 {
            boat.ring_timer -= time.delta_seconds();
            if boat.ring_timer <= 0.0 {
                boat.ring_timer = RING_SPAWN_INTEVAL;
                ev_water_ring_spawn.send(WaterRingSpawnEvent {
                    position: global_transform.translation().truncate(),
                    scale: transform.scale,
                    angle: Vec2::X.angle_between(boat.facing.to_vec()),
                    face: boat.facing,
                });
            }
        } else {
            boat.ring_timer = RING_SPAWN_INTEVAL; // reset
        }
    }
}

fn boat_attack(
    mut query: Query<(
        &mut Boat,
        &mut ForwardCannons,
        &mut ShotgunCannons,
        &mut Shockwave,
        &mut Bombs,
        &mut Kraken,
        &mut DashAttack,
    )>,
    time: Res<Time>,
    cutscenes: Res<Cutscenes>,
) {
    if cutscenes.running() {
        return;
    }
    for (
        mut boat,
        mut forward_cannons,
        mut shotgun_cannons,
        mut shockwave,
        mut bombs,
        mut kraken,
        mut dash,
    ) in query.iter_mut()
    {
        boat.shoot_cooldown += time.delta_seconds();
        if boat.shoot && boat.shoot_cooldown > boat.shoot_cooldown_threshold {
            boat.shoot_cooldown = 0.;
            if boat.attacks.forward_cannons > 0 {
                forward_cannons.level = ForwardCannonsLevel(boat.attacks.forward_cannons);
                forward_cannons.shoot = true;
            }
            if boat.attacks.shotgun_cannons > 0 {
                shotgun_cannons.level = ShotgunCannonsLevel(boat.attacks.shotgun_cannons);
                shotgun_cannons.shoot = true
            }
            if boat.attacks.shockwave > 0 {
                shockwave.level = ShockwaveLevel(boat.attacks.shockwave);
                shockwave.shoot = true
            }
            if boat.attacks.bombs > 0 {
                bombs.level = BombsLevel(boat.attacks.bombs);
                bombs.shoot = true
            }
            if boat.attacks.kraken > 0 {
                kraken.level = KrakenLevel(boat.attacks.kraken);
                kraken.shoot = true
            }
        }
        boat.dash_cooldown -= time.delta_seconds();
        if boat.dash && boat.dash_cooldown <= 0. {
            boat.dash_cooldown = 0.6;
            dash.shoot = true
        }
        boat.dash = false;
    }
}

fn boat_debug(
    mut egui_query: Query<&mut EguiContext>,
    mut menu_bar: ResMut<MenuBar>,
    mut query: Query<(&mut Boat, &Label)>,
) {
    menu_bar.item("Boats", |open| {
        let Some(mut egui_context) = egui_query.get_single_mut().ok() else { return };
        egui::Window::new("Boats")
            .open(open)
            .show(egui_context.get_mut(), |ui| {
                for (mut boat, label) in query.iter_mut() {
                    ui.label(&label.0);
                    ui.horizontal(|ui| {
                        ui.label("Speed");
                        ui.add(egui::Slider::new(&mut boat.speed, 0.0..=1000.0));
                    });
                    ui.label(format!("Facing: {:?}", boat.facing));
                }
            });
    });
}
