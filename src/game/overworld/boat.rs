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
                    .before(PlayerSystems::Camera),
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
            .insert_bundle(SpriteBundle {
                texture: asset_library.sprite_ship.clone(),
                ..Default::default()
            })
            .insert(
                Transform2::from_translation(event.position)
                    .with_depth((DepthLayer::Entity, 0.))
                    .with_scale(Vec2::new(0.5, 0.5)),
            )
            .insert(Boat {
                movement: Vec2::ZERO,
                speed: 200.,
                shoot: false,
            })
            .insert(YDepth::default());
    }
}

fn boat_update(
    mut query: Query<(&mut Transform2, &GlobalTransform, &mut Boat)>,
    time: Res<Time>,
    mut ev_cannon_ball_spawn: EventWriter<CannonBallSpawnEvent>,
) {
    for (mut transform, global_transform, mut boat) in query.iter_mut() {
        if boat.movement.length_squared() > 0. {
            let movement = boat.movement.normalize() * time.delta_seconds();
            transform.translation += movement * boat.speed;
        }
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
                }
            });
    });
}
