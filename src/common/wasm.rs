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
fn wasm_fullscreen(mut window_query: Query<&mut Window>) {
    if let Some(mut window) = window_query.get_single_mut().ok() {
        let web_window = web_sys::window().unwrap();
        let document_element = web_window.document().unwrap().document_element().unwrap();
        let client_width = document_element.client_width() as f32;
        let client_height = document_element.client_height() as f32;
        let (width, height) = if client_width * 768. / 1280. < client_height {
            (client_width, client_width * 768. / 1280.)
        } else {
            (client_height * 1280. / 768., client_height)
        };
        window.resolution.set(width, height);
    }
}
