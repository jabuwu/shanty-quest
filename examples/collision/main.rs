use asset_struct::prelude::*;
use bevy::{prelude::*, window::WindowResolution};
use jam::common::prelude::*;

#[derive(Component)]
pub struct Editable;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Collision".to_string(),
                resolution: WindowResolution::new(1280., 720.),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(jam::common::CommonPlugin)
        .add_startup_system(init)
        .add_system(player_update)
        .run();
}

#[derive(Component)]
pub struct Player;

pub fn init(
    mut commands: Commands,
    mut asset_library: ResMut<AssetLibrary>,
    asset_server: Res<AssetServer>,
) {
    asset_library.load_assets(&asset_server);
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Vec2::new(32., 32.).into(),
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
        .insert(Player);
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Vec2::new(32., 32.).into(),
                color: Color::RED,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Transform2::from_xy(100., 0.))
        .insert(Collision {
            shape: CollisionShape::Rect {
                size: Vec2::new(32., 32.),
            },
            flags: 1,
        });
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Vec2::new(32., 32.).into(),
                color: Color::RED,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Transform2::from_xy(30., 40.))
        .insert(Collision {
            shape: CollisionShape::Rect {
                size: Vec2::new(32., 32.),
            },
            flags: 1,
        });
}

fn player_update(
    mut query: Query<(Entity, &mut Transform2, &Collision), With<Player>>,
    collision_query: Res<CollisionQuery>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (entity, mut transform, collision) in query.iter_mut() {
        let mut velocity = Vec2::new(0., 0.);
        if input.pressed(KeyCode::W) {
            velocity.y += 1.;
        }
        if input.pressed(KeyCode::S) {
            velocity.y -= 1.;
        }
        if input.pressed(KeyCode::A) {
            velocity.x -= 1.;
        }
        if input.pressed(KeyCode::D) {
            velocity.x += 1.;
        }
        if velocity.length_squared() > 0. {
            velocity = velocity.normalize() * 200. * time.delta_seconds();
            let mut velocity_x = Vec2::X * velocity;
            let mut velocity_y = Vec2::Y * velocity;
            let collision_filters = CollisionFilter {
                exclude_entity: entity,
                flags: 1,
            };
            if let Some((_, distance)) = collision_query.check_moving(
                transform.translation,
                velocity_x,
                collision.shape,
                Some(collision_filters),
            ) {
                velocity_x *= distance;
                velocity_x.x -= 0.001_f32.copysign(velocity_x.x);
            }
            transform.translation += velocity_x;
            if let Some((_, distance)) = collision_query.check_moving(
                transform.translation,
                velocity_y,
                collision.shape,
                Some(collision_filters),
            ) {
                velocity_y *= distance;
                velocity_y.y -= 0.001_f32.copysign(velocity_y.y);
            }
            transform.translation += velocity_y;
        }
    }
}
