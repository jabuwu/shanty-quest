use bevy::prelude::*;
use jam::{
    common::prelude::*,
    game::overworld::{
        camera::OverworldCameraPlugin,
        character_controller::{CharacterController, CharacterControllerPlugin},
    },
};

#[derive(Component)]
pub struct Editable;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Angle".to_string(),
            width: 1280.,
            height: 720.,
            resizable: false,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins)
        .add_plugin(CommonPlugin)
        .add_plugin(CharacterControllerPlugin)
        .add_plugin(OverworldCameraPlugin)
        .add_startup_system(init)
        .add_system(update)
        .run();
}

#[derive(Component)]
pub struct PointToMouse;

pub fn init(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Vec2::new(64., 8.).into(),
                color: Color::GREEN,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Transform2::from_xy(0., 0.))
        .insert(Collision {
            shape: CollisionShape::Rect {
                size: Vec2::new(32., 32.),
            },
            flags: 1,
        })
        .insert(CharacterController {
            movement: Vec2::ZERO,
            speed: 300.,
            ..Default::default()
        })
        .insert(PointToMouse);
}

fn update(mut query: Query<&mut Transform2, With<PointToMouse>>, mouse: Res<Mouse>) {
    for mut transform in query.iter_mut() {
        let difference = (mouse.position - transform.translation).normalize();
        if difference.length() > 0. {
            let rotation = Vec2::from_angle(transform.rotation).angle_between(difference);
            transform.rotation = 0.1_f32.lerp(transform.rotation, transform.rotation + rotation);
        }
    }
}
