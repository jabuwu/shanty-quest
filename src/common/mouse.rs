use bevy::prelude::*;

pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Mouse>()
            .add_systems(PreUpdate, mouse_position);
    }
}

#[derive(Default, Resource)]
pub struct Mouse {
    pub position: Vec2,
}

fn mouse_position(
    mut mouse: ResMut<Mouse>,
    window_query: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
) {
    if let Some(window) = window_query.get_single().ok() {
        if let Some(position) = window.cursor_position() {
            if let Ok((camera, camera_transform)) = camera.get_single() {
                let window_size = Vec2::new(window.width() as f32, window.height() as f32);
                let ndc = ((position / window_size) * 2.0 - Vec2::ONE) * Vec2::new(1., -1.);
                let ndc_to_world =
                    camera_transform.compute_matrix() * camera.projection_matrix().inverse();
                let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
                let world_pos: Vec2 = world_pos.truncate();
                mouse.position = world_pos;
            }
        }
    }
}
