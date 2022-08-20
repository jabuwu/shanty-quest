use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use std::collections::HashMap;

pub struct MenuBarPlugin;

impl Plugin for MenuBarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MenuBar>().add_system(menu_bar);
    }
}

#[derive(Default)]
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
    mut egui_context: ResMut<EguiContext>,
    mut menu_bar: ResMut<MenuBar>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Grave) {
        menu_bar.opened = !menu_bar.opened;
    }
    if menu_bar.opened {
        egui::TopBottomPanel::top("top_panel").show(egui_context.ctx_mut(), |ui| {
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
