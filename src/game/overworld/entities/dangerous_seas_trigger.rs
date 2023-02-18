use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;

pub struct DangerousSeasTriggerPlugin;

impl Plugin for DangerousSeasTriggerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(dangerous_seas_world_spawn)
            .add_system(dangerous_seas_check);
    }
}

#[derive(Component)]
pub struct DangerousSeasTrigger;

fn dangerous_seas_world_spawn(
    mut ev_spawn: EventReader<WorldLocationsSpawnEvent>,
    mut commands: Commands,
    world_locations: Res<WorldLocations>,
) {
    for _ in ev_spawn.iter() {
        let triggers = world_locations.get_multiple_rect("DangerousSeasTrigger");
        for trigger in triggers {
            commands
                .spawn(TransformBundle::default())
                .insert(
                    Transform2::from_translation(trigger.position)
                        .with_depth((DepthLayer::Entity, 0.)),
                )
                .insert(Trigger::new(CollisionShape::Rect { size: trigger.size }))
                .insert(DangerousSeasTrigger);
        }
    }
}

fn dangerous_seas_check(
    query: Query<&Trigger, With<DangerousSeasTrigger>>,
    mut game_state: ResMut<GameState>,
    mut ev_cutscene: EventWriter<CutsceneStartEvent<DangerousSeasCutscene>>,
) {
    for trigger in query.iter() {
        if trigger.triggered() {
            if !game_state.dangerous_seas && !game_state.quests.block_dangerous_seas() {
                game_state.dangerous_seas = true;
                ev_cutscene.send_default();
            }
        }
    }
}
