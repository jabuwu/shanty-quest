use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

const RING_SPAWN_INTEVAL: f32 = 0.15;

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
            .add_system(boat_jam)
            .add_system(boat_debug);
    }
}

#[derive(Clone, Copy)]
pub struct BoatSpawnEvent {
    pub entity: Option<Entity>,
    pub position: Vec2,
    pub attack: Attack,
}

#[derive(Component)]
pub struct Boat {
    pub movement: Vec2,
    pub speed: f32,
    pub facing: Facing,
    pub ring_timer: f32,
    pub attack: Attack,
}

#[derive(Component)]
pub struct BoatSprite;

fn boat_spawn(
    mut ev_spawn: EventReader<BoatSpawnEvent>,
    mut commands: Commands,
    mut _ev_healthbar_spawn: EventWriter<HealthbarSpawnEvent>,
    asset_library: Res<AssetLibrary>,
) {
    for event in ev_spawn.iter() {
        let sprite_entity = commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: asset_library.sprite_ship_atlas.clone(),
                ..Default::default()
            })
            .insert(
                Transform2::new()
                    .with_depth((DepthLayer::Entity, 0.))
                    .with_scale(Vec2::new(0.6, 0.6)),
            )
            .insert(BoatSprite)
            .id();
        let mut boat_entity = if let Some(entity) = event.entity {
            commands.entity(entity)
        } else {
            commands.spawn()
        };
        boat_entity
            .insert_bundle(TransformBundle::default())
            .insert_bundle(VisibilityBundle::default())
            .insert(Transform2::from_translation(event.position))
            .insert(Boat {
                movement: Vec2::ZERO,
                speed: 200.,
                facing: Facing::East,
                ring_timer: RING_SPAWN_INTEVAL,
                attack: event.attack,
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
            .insert(Health::new(100.))
            .insert(ShotgunCannons::default())
            .insert(Shockwave::default())
            .insert(DashAttack::default())
            .add_child(sprite_entity);
        /*ev_healthbar_spawn.send(HealthbarSpawnEvent {
            entity: Some(boat_entity.id()),
            offset: Vec2::new(0., 125.),
        });*/
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
        character_controller.movement = boat.movement;
        character_controller.speed = boat.speed;
        if let Some(facing) = Facing::from_vec(boat.movement) {
            boat.facing = facing;
        }
        for child in children.iter() {
            if let Ok(mut atlas) = children_query.get_mut(*child) {
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

        if boat.movement.length_squared() > 0. {
            boat.ring_timer -= time.delta_seconds();
            if boat.ring_timer <= 0.0 {
                boat.ring_timer = RING_SPAWN_INTEVAL;
                ev_water_ring_spawn.send(WaterRingSpawnEvent {
                    entity: None,
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

fn boat_jam(
    mut query: Query<
        (
            &mut BandJam,
            &Boat,
            &mut ShotgunCannons,
            &mut Shockwave,
            &mut DashAttack,
            &Children,
        ),
        With<Boat>,
    >,
    mut children_query: Query<&mut Transform2, With<BoatSprite>>,
) {
    for (mut band_jam, boat, mut shotgun_cannons, mut shockwave, mut dash_attack, children) in
        query.iter_mut()
    {
        for child in children.iter() {
            if let Ok(mut transform) = children_query.get_mut(*child) {
                transform.scale = Vec2::new(0.6, 0.6) + Vec2::new(0.1, 0.1) * band_jam.intensity;
            }
        }
        if band_jam.cannons {
            match boat.attack {
                Attack::ShotgunCannons => shotgun_cannons.shoot = true,
                Attack::Shockwave => shockwave.shoot = true,
                Attack::DashAttack => dash_attack.shoot = true,
            }
            band_jam.cannons = false;
        }
    }
}

fn boat_debug(
    mut egui_context: ResMut<EguiContext>,
    mut menu_bar: ResMut<MenuBar>,
    mut query: Query<(&mut Boat, &Label, Entity)>,
) {
    menu_bar.item("Boats", |open| {
        egui::Window::new("Boats")
            .open(open)
            .show(egui_context.ctx_mut(), |ui| {
                for (mut boat, label, entity) in query.iter_mut() {
                    ui.label(&label.0);
                    ui.horizontal(|ui| {
                        ui.label("Speed");
                        ui.add(egui::Slider::new(&mut boat.speed, 0.0..=1000.0));
                    });
                    ui.label(format!("Facing: {:?}", boat.facing));
                    ui.label("Attack");
                    egui::ComboBox::new(format!("{}", entity.id()), "")
                        .selected_text(format!("{:?}", boat.attack))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut boat.attack,
                                Attack::ShotgunCannons,
                                "ShotgunCannons",
                            );
                            ui.selectable_value(&mut boat.attack, Attack::Shockwave, "Shockwave");
                            ui.selectable_value(&mut boat.attack, Attack::DashAttack, "DashAttack");
                        });
                }
            });
    });
}
