use crate::common::prelude::*;
use bevy::prelude::*;

pub struct DamageFlashPlugin;

impl Plugin for DamageFlashPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DamageFlashSpawnEvent>()
            .add_systems(Update, damage_flash_spawn);
    }
}

#[derive(Event, Default, Clone, Copy)]
pub struct DamageFlashSpawnEvent;

fn damage_flash_spawn(mut ev_spawn: EventReader<DamageFlashSpawnEvent>, mut commands: Commands) {
    for _ in ev_spawn.iter() {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Vec2::new(9999., 9999.).into(),
                    color: Color::rgba(1., 0., 0., 0.12),
                    ..Default::default()
                },
                ..Default::default()
            },
            Transform2::new().with_depth(DEPTH_LAYER_DAMAGE_FLASH),
            TimeToLive { seconds: 0.1 },
            FollowCamera { offset: Vec2::ZERO },
        ));
    }
}
