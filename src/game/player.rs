use crate::common::prelude::*;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerSpawnEvent>()
            .add_system(player_spawn)
            .add_system(player_move)
            .add_system(player_debug);
    }
}

#[derive(Default, Clone, Copy)]
pub struct PlayerSpawnEvent;

#[derive(Component)]
pub struct Player {
    speed: f32,
}

fn player_spawn(mut ev_spawn: EventReader<PlayerSpawnEvent>, mut commands: Commands) {
    for _ in ev_spawn.iter() {
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Vec2::new(32., 32.).into(),
                    color: Color::RED,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Player { speed: 300. });
    }
}

fn player_move(
    mut query: Query<(&mut Transform, &Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    if query.is_empty() {
        return;
    }
    let mut movement = Vec2::ZERO;
    if input.pressed(KeyCode::W) {
        movement.y += 1.;
    }
    if input.pressed(KeyCode::S) {
        movement.y -= 1.;
    }
    if input.pressed(KeyCode::A) {
        movement.x -= 1.;
    }
    if input.pressed(KeyCode::D) {
        movement.x += 1.;
    }
    if movement.length_squared() > 0. {
        movement = movement.normalize() * time.delta_seconds();
        for (mut transform, player) in query.iter_mut() {
            transform.translation += movement.extend(0.) * player.speed;
        }
    }
}

fn player_debug(
    mut egui_context: ResMut<EguiContext>,
    mut menu_bar: ResMut<MenuBar>,
    mut query: Query<&mut Player>,
) {
    menu_bar.item("Player", |open| {
        egui::Window::new("Player")
            .open(open)
            .show(egui_context.ctx_mut(), |ui| {
                for mut player in query.iter_mut() {
                    ui.horizontal(|ui| {
                        ui.label("Speed");
                        ui.add(egui::Slider::new(&mut player.speed, 0.0..=1000.0));
                    });
                }
            });
    });
}
