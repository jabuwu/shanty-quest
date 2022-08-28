use crate::common::prelude::*;
use bevy::prelude::*;

const VIGNETTE_INTENSITY: f32 = 0.5;

pub struct VignettePlugin;

impl Plugin for VignettePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<VignetteSpawnEvent>()
            .add_system(vignette_spawn);
    }
}

#[derive(Default, Clone, Copy)]
pub struct VignetteSpawnEvent;

fn vignette_spawn(
    mut ev_spawn: EventReader<VignetteSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for _ in ev_spawn.iter() {
        commands
            .spawn_bundle(VisibilityBundle::default())
            .insert_bundle(TransformBundle::default())
            .insert(FollowCamera { offset: Vec2::ZERO })
            .insert(Transform2::new().without_pixel_perfect())
            .with_children(|parent| {
                parent
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            custom_size: Vec2::new(1280., 768.).into(),
                            color: Color::rgba(0., 0., 0., VIGNETTE_INTENSITY),
                            ..Default::default()
                        },
                        texture: asset_library.sprite_screen_edges.clone(),
                        ..Default::default()
                    })
                    .insert(
                        Transform2::from_xy(0., 0.)
                            .with_depth(DEPTH_LAYER_VIGNETTE)
                            .without_pixel_perfect(),
                    );
            });
    }
}
