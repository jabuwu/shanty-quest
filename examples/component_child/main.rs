use bevy::prelude::*;
use jam::common::prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Component Child".to_string(),
            width: 1280.,
            height: 720.,
            resizable: false,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins)
        .add_component_child::<MyComponent, MyMarker>()
        .add_plugin(CommonPlugin)
        .add_startup_system(init)
        .add_system(create_entities)
        .add_system(destroy_entities)
        .add_system(spawn_child)
        .run();
}

#[derive(Component)]
pub struct MyComponent;

#[derive(Default, Component)]
pub struct MyMarker;

fn init(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn create_entities(mut commands: Commands, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::C) {
        commands
            .spawn_bundle(VisibilityBundle::default())
            .insert_bundle(TransformBundle::default())
            .insert(Transform2::from_xy(
                rand::random::<f32>() * 600. - 300.,
                rand::random::<f32>() * 600. - 300.,
            ))
            .insert(MyComponent);
    }
}

fn destroy_entities(
    mut commands: Commands,
    query: Query<Entity, With<MyComponent>>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::D) {
        for entity in query.iter() {
            if rand::random() {
                commands.entity(entity).remove::<MyComponent>();
            } else {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

fn spawn_child(
    mut commands: Commands,
    mut ev_child_created: EventReader<ComponentChildCreatedEvent<MyMarker>>,
) {
    for event in ev_child_created.iter() {
        commands.entity(event.entity).insert_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Vec2::new(16., 16.).into(),
                color: Color::RED,
                ..Default::default()
            },
            ..Default::default()
        });
    }
}
