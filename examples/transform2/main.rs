use bevy::prelude::*;
use jam::common::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Player".to_string(),
                width: 1280.,
                height: 720.,
                resizable: false,
                ..default()
            },
            ..default()
        }))
        .add_plugin(CommonPlugin)
        .add_startup_system(init)
        .add_system(move_red)
        .run();
}

#[derive(Component)]
pub struct Red;
#[derive(Component)]
pub struct Blue;

pub fn init(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Vec2::new(64., 64.).into(),
                color: Color::RED,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Transform2::from_xy(0., 0.))
        .insert(Red)
        .insert(YDepth::default());
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Vec2::new(64., 64.).into(),
                color: Color::BLUE,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Transform2::from_xy(0., 0.))
        .insert(Blue)
        .insert(YDepth::default());
}

pub fn move_red(
    mut query: Query<&mut Transform2, With<Red>>,
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
) {
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
    }
    if movement.length_squared() > 0. {
        movement = movement.normalize() * 300. * time.delta_seconds();
    }
    for mut transform in query.iter_mut() {
        transform.translation += movement;
    }
}
