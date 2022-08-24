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

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreatLevel {
    #[default]
    None,
    Easy,
    Medium,
}

fn threat_level_update(
    player_query: Query<&GlobalTransform, With<Player>>,
    mut threat_level: ResMut<ThreatLevel>,
    world_locations: Res<WorldLocations>,
) {
    let player_position = if let Ok(player_transform) = player_query.get_single() {
        player_transform.translation().truncate()
    } else {
        *threat_level = ThreatLevel::None;
        return;
    };
    *threat_level = ThreatLevel::None;
    for rect in world_locations.get_multiple_rect("ThreatLevelEasy").iter() {
        if (CollisionShape::Rect { size: rect.size }).overlaps(
            rect.position,
            CollisionShape::Point,
            player_position,
        ) {
            *threat_level = ThreatLevel::Easy;
            return;
        }
    }
    for rect in world_locations
        .get_multiple_rect("ThreatLevelMedium")
        .iter()
    {
        if (CollisionShape::Rect { size: rect.size }).overlaps(
            rect.position,
            CollisionShape::Point,
            player_position,
        ) {
            *threat_level = ThreatLevel::Medium;
            return;
        }
    }
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
