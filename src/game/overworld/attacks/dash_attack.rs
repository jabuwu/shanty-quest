use crate::game::prelude::*;
use bevy::prelude::*;

pub struct DashAttackPlugin;

impl Plugin for DashAttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(dash_attack_fire)
            .add_system(dash_attack_update);
    }
}

#[derive(Component, Default)]
pub struct DashAttack {
    pub shoot: bool,
}

#[derive(Component)]
pub struct Dash {
    pub velocity: Vec2,
    pub time_alive: f32,
}

fn dash_attack_fire(
    mut query: Query<(&mut DashAttack, &Boat, Entity), Without<Dash>>,
    mut commands: Commands,
) {
    for (mut dash_attack, boat, entity) in query.iter_mut() {
        if dash_attack.shoot {
            commands.entity(entity).insert(Dash {
                velocity: boat.facing.to_vec() * 1000.,
                time_alive: 0.,
            });
            dash_attack.shoot = false;
        }
    }
}

fn dash_attack_update(
    mut query: Query<(&mut Dash, Entity)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (mut dash, entity) in query.iter_mut() {
        dash.time_alive += time.delta_seconds();
        if dash.time_alive > 0.25 {
            commands.entity(entity).remove::<Dash>();
        }
    }
}
