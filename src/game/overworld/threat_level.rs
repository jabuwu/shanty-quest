use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

pub struct ThreatLevelPlugin;

impl Plugin for ThreatLevelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ThreatLevel>()
            .add_system(threat_level_update)
            .add_system(threat_level_debug);
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Resource)]
pub enum ThreatLevel {
    #[default]
    None,
    Easy,
    Medium,
    Hard,
    Midnight,
    Davy,
}

fn threat_level_update(
    player_query: Query<&GlobalTransform, With<Player>>,
    mut threat_level: ResMut<ThreatLevel>,
    world_locations: Res<WorldLocations>,
    game_state: Res<GameState>,
) {
    let player_position = if let Ok(player_transform) = player_query.get_single() {
        player_transform.translation().truncate()
    } else {
        *threat_level = ThreatLevel::None;
        return;
    };
    *threat_level = ThreatLevel::None;

    macro_rules! disable_threat_level_near_position {
        ($str:literal, $condition:expr) => {
            if $condition
                && player_position.distance(world_locations.get_single_position($str)) < 500.
            {
                return;
            }
        };
    }

    if game_state.quests.davy() && game_state.quests.fighting() {
        *threat_level = ThreatLevel::Davy;
        return;
    }

    disable_threat_level_near_position!("JagerossaTrigger", game_state.quests.jagerossa());
    disable_threat_level_near_position!("RingoTrigger", game_state.quests.ringo());
    disable_threat_level_near_position!("PlankTrigger", game_state.quests.plank());
    disable_threat_level_near_position!("DavyTrigger", game_state.quests.davy());

    macro_rules! threat_level {
        ($str:literal, $value:expr) => {
            for rect in world_locations.get_multiple_rect($str).iter() {
                if (CollisionShape::Rect { size: rect.size }).overlaps(
                    rect.position,
                    CollisionShape::Point,
                    player_position,
                ) {
                    *threat_level = $value;
                    return;
                }
            }
        };
    }
    threat_level!("ThreatLevelEasy", ThreatLevel::Easy);
    threat_level!("ThreatLevelMedium", ThreatLevel::Medium);
    threat_level!("ThreatLevelHard", ThreatLevel::Hard);
    threat_level!("ThreatLevelMidnight", ThreatLevel::Midnight);
}

fn threat_level_debug(
    mut egui_context: ResMut<EguiContext>,
    mut menu_bar: ResMut<MenuBar>,
    threat_level: Res<ThreatLevel>,
) {
    menu_bar.item("Threat Levels", |open| {
        egui::Window::new("Threat Levels")
            .open(open)
            .show(egui_context.ctx_mut(), |ui| {
                ui.label(format!("Threat level: {:?}", threat_level.as_ref()));
            });
    });
}
