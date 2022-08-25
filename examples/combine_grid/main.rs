use bevy::prelude::*;
use grid_combiner::{GridCombiner, GridPoint};

fn random_color() -> Color {
    Color::rgb(
        rand::random::<f32>(),
        rand::random::<f32>(),
        rand::random::<f32>(),
    )
}

#[derive(Default)]
struct State {
    combiner: Option<GridCombiner>,
}

fn main() {
    App::new()
        .init_resource::<State>()
        .add_plugins(DefaultPlugins)
        .add_startup_system(init)
        .add_startup_system(draw_combiner)
        .add_system(draw_combined)
        .run();
}

fn init(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn draw_combiner(mut commands: Commands, mut state: ResMut<State>) {
    let mut combiner = GridCombiner::new();
    let size = 10;
    while combiner.points().len() < 250 {
        combiner.add_point(GridPoint::new(
            (rand::random::<i64>() % size) - size / 2,
            (rand::random::<i64>() % size) - size / 2,
        ));
    }
    for point in combiner.points().iter() {
        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: (Vec2::ONE * 16.).into(),
                color: Color::RED,
                ..Default::default()
            },
            transform: Transform::from_translation(
                Vec2::new(point.x as f32 * 16., point.y as f32 * 16.).extend(0.0),
            ),
            ..Default::default()
        });
    }
    state.combiner = Some(combiner.clone());
}

fn draw_combined(mut commands: Commands, mut state: ResMut<State>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::Space) {
        if let Some(combiner) = &state.combiner {
            let rects = combiner.combine();
            for rect in rects.iter() {
                let (pos, size) = rect.to_position_size();
                commands.spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        custom_size: (size * 16.).into(),
                        color: random_color(),
                        ..Default::default()
                    },
                    transform: Transform::from_translation((pos * 16.).extend(0.1)),
                    ..Default::default()
                });
            }
        }
        state.combiner = None;
    }
}
