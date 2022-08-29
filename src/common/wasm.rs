use bevy::prelude::*;

pub struct WasmPlugin;

#[cfg(not(target_arch = "wasm32"))]
impl Plugin for WasmPlugin {
    fn build(&self, _app: &mut App) {}
}

#[cfg(target_arch = "wasm32")]
impl Plugin for WasmPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(wasm_fullscreen);
    }
}

#[cfg(target_arch = "wasm32")]
fn wasm_fullscreen(mut windows: ResMut<Windows>) {
    if let Some(window) = windows.get_primary_mut() {
        let web_window = web_sys::window().unwrap();
        let document_element = web_window.document().unwrap().document_element().unwrap();
        let client_width = document_element.client_width() as f32;
        let client_height = document_element.client_height() as f32;
        let (width, height) = if client_width * 768. / 1280. < client_height {
            (client_width, client_width * 768. / 1280.)
        } else {
            (client_height * 1280. / 768., client_height)
        };
        window.set_resolution(width, height);
    }
}
