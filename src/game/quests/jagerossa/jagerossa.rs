use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;

pub struct JagerossaPlugin;

impl Plugin for JagerossaPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<JagerossaSpawnEvent>()
            .add_system(jagerossa_spawn)
            .add_system(jagerossa_move)
            .add_system(jagerossa_invincibility);
    }
}

#[derive(Default, Clone, Copy)]
pub struct JagerossaSpawnEvent;

#[derive(Component)]
pub struct Jagerossa;

fn jagerossa_spawn(
    mut ev_spawn: EventReader<JagerossaSpawnEvent>,
    mut ev_boat_spawn: EventWriter<BoatSpawnEvent>,
    mut commands: Commands,
) {
    for _ in ev_spawn.iter() {
        let entity = commands
            .spawn()
            .insert(Jagerossa)
            .insert(Label("Jagerossa".to_owned()))
            .insert(AutoDamage {
                despawn: true,
                ..Default::default()
            })
            .id();
        ev_boat_spawn.send(BoatSpawnEvent {
            entity: Some(entity),
            position: Vec2::new(200., -200.),
            special_attack: SpecialAttack::ShotgunCannons,
            healthbar: true,
        });
    }
}

fn jagerossa_move(mut query: Query<&mut Boat, With<Jagerossa>>) {
    for mut boat in query.iter_mut() {
        boat.movement = Vec2::ZERO;
        boat.direction = std::f32::consts::PI;
    }
}

fn jagerossa_invincibility(mut query: Query<(&mut Boat, &AutoDamage), With<Jagerossa>>) {
    for (mut boat, auto_damage) in query.iter_mut() {
        boat.opacity = if auto_damage.invincibility > 0. {
            0.5
        } else {
            1.
        };
    }
}
