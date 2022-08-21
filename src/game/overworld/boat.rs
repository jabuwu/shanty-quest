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
    pub shoot: bool,
    pub facing: Facing,
    pub ring_timer: f32,
}

fn boat_spawn(
    mut ev_spawn: EventReader<BoatSpawnEvent>,
    mut commands: Commands,
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
                    .with_scale(Vec2::new(0.75, 0.75)),
            )
            .insert(Boat {
                movement: Vec2::ZERO,
                speed: 200.,
                shoot: false,
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
            });
    }
}

fn boat_update(
    mut query: Query<(
        &mut Transform2,
        &mut CharacterController,
        &GlobalTransform,
        &mut Boat,
        &mut TextureAtlasSprite,
    )>,
    time: Res<Time>,
    mut ev_cannon_ball_spawn: EventWriter<CannonBallSpawnEvent>,
    mut ev_water_ring_spawn: EventWriter<WaterRingSpawnEvent>,
) {
    for (mut transform, mut character_controller, global_transform, mut boat, mut atlas) in
        query.iter_mut()
    {
        character_controller.movement = boat.movement;
        character_controller.speed = boat.speed;
        if boat.shoot {
            boat.shoot = false;
            ev_cannon_ball_spawn.send(CannonBallSpawnEvent {
                entity: None,
                position: global_transform.translation().truncate(),
                velocity: Vec2::X * 700.,
            });
            ev_cannon_ball_spawn.send(CannonBallSpawnEvent {
                entity: None,
                position: global_transform.translation().truncate(),
                velocity: Vec2::X * -700.,
            });
        }
        transform.scale.x = transform.scale.x.abs();
        if let Some(facing) = Facing::from_vec(boat.movement) {
            boat.facing = facing;
        }
        match boat.facing {
            Facing::North => {
                atlas.index = 3;
            }
            Facing::NorthEast => {
                atlas.index = 1;
                transform.scale.x *= -1.;
            }
            Facing::East => {
                atlas.index = 2;
                transform.scale.x *= -1.;
            }
            Facing::SouthEast => {
                atlas.index = 0;
                transform.scale.x *= -1.;
            }
            Facing::South => {
                atlas.index = 4;
            }
            Facing::SouthWest => {
                atlas.index = 0;
            }
            Facing::West => {
                atlas.index = 2;
            }
            Facing::NorthWest => {
                atlas.index = 1;
            }
        }

        if boat.movement.length_squared() > 0. {
            boat.ring_timer -= time.delta_seconds();
            if boat.ring_timer <= 0.0 {
                boat.ring_timer = 0.2;
                ev_water_ring_spawn.send(WaterRingSpawnEvent {
                    entity: None,
                    position: global_transform.translation().truncate(),
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
