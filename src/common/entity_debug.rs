use crate::common::prelude::*;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

pub struct EntityDebugPlugin;

impl Plugin for EntityDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(entity_debug);
    }
}

fn entity_debug(
    mut egui_context: ResMut<EguiContext>,
    mut menu_bar: ResMut<MenuBar>,
    mut query: Query<(&Label, Option<&mut Transform2>)>,
) {
    menu_bar.item("Labeled Entities", |open| {
        egui::Window::new("Labeled Entities")
            .open(open)
            .show(egui_context.ctx_mut(), |ui| {
                for (label, mut transform2) in query.iter_mut() {
                    ui.label(&label.0);
                    if let Some(transform2) = transform2.as_mut() {
                        ui.horizontal(|ui| {
                            ui.label("X");
                            ui.add(
                                egui::DragValue::new(&mut transform2.translation.x)
                                    .speed(1)
                                    .clamp_range(-99999.0..=99999.0),
                            );

                            ui.label("Y");
                            ui.add(
                                egui::DragValue::new(&mut transform2.translation.y)
                                    .speed(1)
                                    .clamp_range(-99999.0..=99999.0),
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.label(format!(
                                "Depth Layer: {:?} ({}) = {}",
                                transform2.depth_layer,
                                transform2.depth,
                                transform2.depth_f32()
                            ));
                        });
                    }
                }
            });
    });
}
