use crate::game::prelude::*;
use bevy::prelude::*;

use self::controls::ControlsUiSpawnEvent;
use self::experience::ExperienceUiSpawnEvent;
use self::health::HealthUiSpawnEvent;
use self::health_aura::HealthAuraSpawnEvent;
use self::marker::MarkerSpawnEvent;
use self::objective::ObjectiveSpawnEvent;
use self::town_marker::TownMarkerSpawnEvent;
use self::vignette::VignetteSpawnEvent;

pub struct OverworldUiPlugin;

impl Plugin for OverworldUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OverworldUiSpawnEvent>()
            .add_plugins((
                map::MapPlugin,
                marker::MarkerPlugin,
                town_marker::TownMarkerPlugin,
                objective::ObjectivePlugin,
                boss_healthbar::BossHealthbarPlugin,
                checkpoint::CheckpointPlugin,
                health::HealthUiPlugin,
                controls::ControlsUiPlugin,
                experience::ExperienceUiPlugin,
                level_up::LevelUpPlugin,
                health_aura::HealthAuraPlugin,
                vignette::VignettePlugin,
            ))
            .add_systems(Update, overworld_ui_spawn);
    }
}

#[derive(Event, Default, Clone, Copy)]
pub struct OverworldUiSpawnEvent;

fn overworld_ui_spawn(
    mut ev_spawn: EventReader<OverworldUiSpawnEvent>,
    mut ev_marker_spawn: EventWriter<MarkerSpawnEvent>,
    mut ev_town_marker_spawn: EventWriter<TownMarkerSpawnEvent>,
    mut ev_objective_spawn: EventWriter<ObjectiveSpawnEvent>,
    mut ev_health_spawn: EventWriter<HealthUiSpawnEvent>,
    mut ev_checkpoint_spawn: EventWriter<CheckpointSpawnEvent>,
    mut ev_controls_spawn: EventWriter<ControlsUiSpawnEvent>,
    mut ev_experience_spawn: EventWriter<ExperienceUiSpawnEvent>,
    mut ev_vignette_spawn: EventWriter<VignetteSpawnEvent>,
    mut ev_health_aura_spawn: EventWriter<HealthAuraSpawnEvent>,
    game_state: Res<GameState>,
) {
    for _ in ev_spawn.iter() {
        ev_marker_spawn.send_default();
        ev_town_marker_spawn.send_default();
        ev_objective_spawn.send_default();
        ev_health_spawn.send_default();
        ev_controls_spawn.send_default();
        ev_experience_spawn.send_default();
        ev_vignette_spawn.send_default();
        ev_health_aura_spawn.send_default();
        if game_state.checkpoint_notification {
            ev_checkpoint_spawn.send_default();
        }
    }
}

pub mod boss_healthbar;
pub mod checkpoint;
pub mod controls;
pub mod experience;
pub mod health;
pub mod health_aura;
pub mod level_up;
pub mod map;
pub mod marker;
pub mod objective;
pub mod town_marker;
pub mod vignette;
