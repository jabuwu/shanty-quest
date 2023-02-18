use asset_struct::prelude::*;
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
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Character Controller".to_string(),
                width: 1280.,
                height: 720.,
                resizable: false,
                ..default()
            },
            ..default()
        }))
        .add_plugin(CommonPlugin)
        .add_plugin(CharacterControllerPlugin)
        .add_plugin(OverworldCameraPlugin)
        .add_startup_system(init)
        .add_system(player_move)
        .add_system(box_move)
        .run();
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct MovingBox {
    right: bool,
}

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
        .insert(Transform2::from_xy(70., 0.))
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
            flags: COLLISION_FLAG,
        })
        .insert(CharacterController {
            movement: Vec2::ZERO,
            speed: 300.,
            ..Default::default()
        })
        .insert(MovingBox { right: true });
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
            flags: COLLISION_FLAG,
        })
        .insert(CharacterController {
            movement: Vec2::ZERO,
            speed: 300.,
            ..Default::default()
        })
        .insert(MovingBox { right: false });
}

fn player_move(
    mut query: Query<&mut CharacterController, With<Player>>,
    input: Res<Input<KeyCode>>,
) {
    for mut character_controller in query.iter_mut() {
        character_controller.movement = Vec2::ZERO;
        if input.pressed(KeyCode::W) {
            character_controller.movement.y += 1.;
        }
        if input.pressed(KeyCode::S) {
            character_controller.movement.y -= 1.;
        }
        if input.pressed(KeyCode::A) {
            character_controller.movement.x -= 1.;
        }
        if input.pressed(KeyCode::D) {
            character_controller.movement.x += 1.;
        }
    }
}

fn box_move(mut query: Query<(&mut CharacterController, &mut MovingBox, &GlobalTransform)>) {
    for (mut character_controller, mut moving_box, transform) in query.iter_mut() {
        character_controller.movement = Vec2::ZERO;
        if moving_box.right {
            character_controller.movement.x += 1.;
            if transform.translation().x > 100. {
                moving_box.right = false;
            }
        } else {
            character_controller.movement.x -= 1.;
            if transform.translation().x < -100. {
                moving_box.right = true;
            }
        }
    }
}
