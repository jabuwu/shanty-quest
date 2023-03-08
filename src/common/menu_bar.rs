use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use std::collections::HashMap;

use crate::DEV_BUILD;

pub struct MenuBarPlugin;

impl Plugin for MenuBarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MenuBar>().add_system(menu_bar);
    }
}

#[derive(Default, Resource)]
pub struct MenuBar {
    opened: bool,
    open: HashMap<String, bool>,
}

impl MenuBar {
    pub fn item<F>(&mut self, label: &str, mut func: F)
    where
        F: FnMut(&mut bool) -> (),
    {
        if let Some(open) = self.open.get_mut(label) {
            if self.opened {
                func(open);
            }
        } else {
            self.open.insert(label.to_owned(), false);
        }
    }
}

pub fn menu_bar(
    mut egui_query: Query<&mut EguiContext>,
    mut menu_bar: ResMut<MenuBar>,
    input: Res<Input<KeyCode>>,
) {
    if DEV_BUILD {
        if input.just_pressed(KeyCode::Grave) {
            menu_bar.opened = !menu_bar.opened;
        }
    }
    if menu_bar.opened {
        let Some(mut egui_context) = egui_query.get_single_mut().ok() else { return };
        egui::TopBottomPanel::top("top_panel").show(egui_context.get_mut(), |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Debug", |ui| {
                    for (label, open) in menu_bar.open.iter_mut() {
                        if ui.button(label).clicked() {
                            *open = !*open;
                            ui.close_menu();
                        }
                    }
                });
            });
        });
    }
}
