use asset_struct::prelude::*;
use bevy::prelude::*;
use jam::common::prelude::*;

#[derive(Component)]
pub struct Editable;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Clickable".to_string(),
            width: 1280.,
            height: 720.,
            resizable: false,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins)
        .add_plugin(CommonPlugin)
        .add_startup_system(init)
        .add_system(update)
        .run();
}

pub fn init(
    mut commands: Commands,
    mut asset_library: ResMut<AssetLibrary>,
    asset_server: Res<AssetServer>,
) {
    asset_library.load_assets(&asset_server);
    commands.spawn_bundle(Camera2dBundle::default());
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Vec2::new(64., 64.).into(),
                color: Color::DARK_GRAY,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Transform2::from_xy(0., 0.))
        .insert(Clickable::new(CollisionShape::Rect {
            size: Vec2::new(64., 64.),
        }));
}

pub fn update(mut query: Query<(&mut Sprite, &mut Transform2, &Clickable)>) {
    for (mut sprite, mut transform, clickable) in query.iter_mut() {
        sprite.color = Color::DARK_GRAY;
        transform.scale = Vec2::ONE;
        if clickable.clicked {
            if clickable.hovered {
                sprite.color = Color::GREEN;
            } else {
                sprite.color = Color::GRAY;
            }
        } else if clickable.hovered {
            sprite.color = Color::YELLOW;
        }
        if clickable.just_hovered() {
            transform.scale = Vec2::ONE * 2.;
        }
        if clickable.just_clicked() {
            transform.scale = Vec2::ONE * 3.;
        }
        if clickable.confirmed {
            transform.scale = Vec2::ONE * 4.;
        }
    }
}
