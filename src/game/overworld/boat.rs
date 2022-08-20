use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

pub struct BoatPlugin;

impl Plugin for BoatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BoatSpawnEvent>()
            .add_system(boat_spawn)
            .add_system(boat_move)
            .add_system(boat_shoot)
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

fn boat_spawn(mut ev_spawn: EventReader<BoatSpawnEvent>, mut commands: Commands) {
    for event in ev_spawn.iter() {
        let mut boat_entity = if let Some(entity) = event.entity {
            commands.entity(entity)
        } else {
            commands.spawn()
        };
        boat_entity
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Vec2::new(32., 48.).into(),
                    color: Color::rgb(0.4, 0.3, 0.1),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(
                Transform2::from_translation(event.position).with_depth((DepthLayer::Entity, 0.)),
            )
            .insert(Boat {
                movement: Vec2::ZERO,
                speed: 150.,
                shoot: false,
            })
            .insert(YDepth::default());
    }
}

fn boat_move(mut query: Query<(&mut Transform2, &Boat)>, time: Res<Time>) {
    for (mut transform, boat) in query.iter_mut() {
        if boat.movement.length_squared() > 0. {
            let movement = boat.movement.normalize() * time.delta_seconds();
            transform.translation += movement * boat.speed;
        }
    }
}

fn boat_shoot(
    mut query: Query<(&mut Boat, &GlobalTransform)>,
    mut ev_cannon_ball_spawn: EventWriter<CannonBallSpawnEvent>,
) {
    for (mut boat, transform) in query.iter_mut() {
        if boat.shoot {
            boat.shoot = false;
            ev_cannon_ball_spawn.send(CannonBallSpawnEvent {
                entity: None,
                position: transform.translation().truncate(),
                velocity: Vec2::X * 700.,
            });
            ev_cannon_ball_spawn.send(CannonBallSpawnEvent {
                entity: None,
                position: transform.translation().truncate(),
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
