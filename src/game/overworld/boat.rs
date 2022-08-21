use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum BoatSystems {
    Update,
}

pub struct BoatPlugin;

impl Plugin for BoatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BoatSpawnEvent>()
            .add_system(boat_spawn)
            .add_system(
                boat_update
                    .label(BoatSystems::Update)
                    .label(CharacterControllerSystems::Update),
            )
            .add_system(boat_debug);
    }
}

#[derive(Default, Clone, Copy)]
pub struct BoatSpawnEvent {
    pub entity: Option<Entity>,
    pub position: Vec2,
}

#[derive(Component)]
pub struct Boat {
    pub movement: Vec2,
    pub speed: f32,
    pub shoot_port: bool,
    pub shoot_starboard: bool,
    pub facing: Facing,
    pub ring_timer: f32,
}

fn boat_spawn(
    mut ev_spawn: EventReader<BoatSpawnEvent>,
    mut commands: Commands,
    mut ev_healthbar_spawn: EventWriter<HealthbarSpawnEvent>,
    asset_library: Res<AssetLibrary>,
) {
    for event in ev_spawn.iter() {
        let mut boat_entity = if let Some(entity) = event.entity {
            commands.entity(entity)
        } else {
            commands.spawn()
        };
        boat_entity
            .insert_bundle(SpriteSheetBundle {
                texture_atlas: asset_library.sprite_ship_atlas.clone(),
                ..Default::default()
            })
            .insert(
                Transform2::from_translation(event.position)
                    .with_depth((DepthLayer::Entity, 0.))
                    .with_scale(Vec2::new(0.6, 0.6)),
            )
            .insert(Boat {
                movement: Vec2::ZERO,
                speed: 200.,
                shoot_port: false,
                shoot_starboard: false,
                facing: Facing::East,
                ring_timer: 0.2,
            })
            .insert(YDepth::default())
            .insert(Collision {
                shape: CollisionShape::Rect {
                    size: Vec2::new(100., 100.),
                },
                flags: COLLISION_FLAG,
            })
            .insert(CharacterController {
                movement: Vec2::ZERO,
                speed: 200.,
            })
            .insert(Health::new(100.));
        ev_healthbar_spawn.send(HealthbarSpawnEvent {
            entity: Some(boat_entity.id()),
            offset: Vec2::new(0., 195.),
        });
    }
}

fn boat_update(
    mut query: Query<(
        &mut CharacterController,
        &GlobalTransform,
        &mut Boat,
        &mut TextureAtlasSprite,
    )>,
    time: Res<Time>,
    mut ev_cannon_ball_spawn: EventWriter<CannonBallSpawnEvent>,
    mut ev_water_ring_spawn: EventWriter<WaterRingSpawnEvent>,
) {
    for (mut character_controller, global_transform, mut boat, mut atlas) in query.iter_mut() {
        character_controller.movement = boat.movement;
        character_controller.speed = boat.speed;
        if let Some(facing) = Facing::from_vec(boat.movement) {
            boat.facing = facing;
        }
        for shoot_side in 0..2 {
            if (shoot_side == 0 && boat.shoot_port) || (shoot_side == 1 && boat.shoot_starboard) {
                let forward = boat.facing.to_vec();
                let mult = if shoot_side == 0 { 1. } else { -1. };
                let side = forward.perp() * mult;
                for i in -1..=1 {
                    let mut angle = Vec2::X.angle_between(side);
                    angle -= std::f32::consts::PI * 0.1 * i as f32 * mult;
                    ev_cannon_ball_spawn.send(CannonBallSpawnEvent {
                        entity: None,
                        position: global_transform.translation().truncate()
                            + forward * 20. * i as f32
                            + side * 50.,
                        velocity: Vec2::from_angle(angle) * 900.,
                    });
                }
            }
        }
        boat.shoot_port = false;
        boat.shoot_starboard = false;
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

        if boat.movement.length_squared() > 0. {
            boat.ring_timer -= time.delta_seconds();
            if boat.ring_timer <= 0.0 {
                boat.ring_timer = 0.2;
                ev_water_ring_spawn.send(WaterRingSpawnEvent {
                    entity: None,
                    position: global_transform.translation().truncate(),
                    scale: transform.scale,
                    angle: Vec2::X.angle_between(boat.facing.to_vec()),
                    face: boat.facing,
                });
            }
        } else {
            boat.ring_timer = 0.1; // reset
        }
    }
}

fn boat_debug(
    mut egui_context: ResMut<EguiContext>,
    mut menu_bar: ResMut<MenuBar>,
    mut query: Query<(&mut Boat, &Label)>,
) {
    menu_bar.item("Boats", |open| {
        egui::Window::new("Boats")
            .open(open)
            .show(egui_context.ctx_mut(), |ui| {
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
