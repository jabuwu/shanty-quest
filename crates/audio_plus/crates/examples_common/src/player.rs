use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(player_move);
    }
}

#[derive(Component)]
pub struct Player;

fn player_move(
    mut query: Query<&mut Transform, With<Player>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
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
        movement = movement.normalize() * 250. * time.delta_seconds();
        for mut transform in query.iter_mut() {
            transform.translation += movement.extend(0.);
        }
    }
}
