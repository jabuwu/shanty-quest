use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EnemySpawnEvent>()
            .add_system(enemy_spawn)
            .add_system(enemy_move);
    }
}

#[derive(Default, Clone, Copy)]
pub struct EnemySpawnEvent {
    pub position: Vec2,
}

#[derive(Component)]
pub struct Enemy;

fn enemy_spawn(
    mut ev_spawn: EventReader<EnemySpawnEvent>,
    mut ev_boat_spawn: EventWriter<BoatSpawnEvent>,
    mut commands: Commands,
) {
    for event in ev_spawn.iter() {
        let entity = commands
            .spawn()
            .insert(Enemy)
            .insert(Label("Enemy".to_owned()))
            .id();
        ev_boat_spawn.send(BoatSpawnEvent {
            entity: Some(entity),
            position: event.position,
            special_attack: SpecialAttack::ShotgunCannons,
            healthbar: true,
        });
    }
}

fn enemy_move(mut query: Query<&mut Boat, With<Enemy>>) {
    for mut boat in query.iter_mut() {
        if boat.movement.length_squared() == 0. {
            boat.movement = Vec2::new(1., 0.);
        }
        let mut angle = Vec2::X.angle_between(boat.movement);
        angle += rand::random::<f32>() * 0.2;
        angle -= rand::random::<f32>() * 0.2;
        boat.movement = Vec2::from_angle(angle);
        boat.movement = Vec2::ZERO;
        boat.direction = angle;
    }
}
